use crate::{
    checker::Checker,
    utils::{Utils, enums::ApiError, structs::TokenResult},
};
use clap::Parser;
use std::{process::exit, time::Duration};
use tokio::{
    task::{JoinError, JoinHandle},
    time::sleep,
};

pub mod api;
mod checker;
pub mod utils;
type CheckTaskOutput = (usize, String, Result<TokenResult, ApiError>);

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Token to check (accepts token/file)
    #[arg(short, long)]
    pub token: String,

    /// Mask last token part for security purposes
    #[arg(short, long, default_value_t = false)]
    pub mask_token: bool,

    /// Maximum check retries for the token
    #[arg(short, long, default_value_t = 5)]
    pub check_retries: u8,

    /// Delay (in seconds) before retrying a request after hitting a rate limit [default: 10]
    #[arg(short, long, value_parser = Utils::parse_duration_secs, default_value = "10")]
    pub ratelimit_retry_delay: Duration,

    /// Delay (in seconds) before retrying a failed network request [default: 5]
    #[arg(short, long, value_parser = Utils::parse_duration_secs, default_value = "5")]
    pub network_retry_delay: Duration,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let tokens_to_check = match Utils::read_tokens_from_input(&args.token) {
        Ok(tokens) => tokens,
        Err(err_msg) => {
            eprintln!("{}", err_msg);
            exit(1);
        }
    };

    let total_tokens = tokens_to_check.len();
    if total_tokens == 0 {
        println!("No tokens to check.");
        return;
    }

    println!("Checking {} token(s)...", total_tokens);

    let mut tasks: Vec<JoinHandle<CheckTaskOutput>> = Vec::new();

    for (index, token) in tokens_to_check.into_iter().enumerate() {
        let current_index = index;

        let task: JoinHandle<CheckTaskOutput> = tokio::spawn(async move {
            let mut final_result: Result<TokenResult, ApiError> = Err(ApiError::IoError(
                std::io::Error::new(std::io::ErrorKind::Other, "Initial loop error"),
            ));

            for attempt in 0..args.check_retries {
                let checker = Checker::new(&token);
                let result = checker.check().await;

                match result {
                    Ok(token_info_result) => {
                        final_result = Ok(token_info_result);
                        break;
                    }
                    Err(ref _reqw_err @ ApiError::RequestError(ref req_err))
                        if req_err.is_timeout() || req_err.is_connect() || req_err.is_request() =>
                    {
                        if attempt < args.check_retries - 1 {
                            eprintln!(
                                " -> [Token #{}/{}] Network error. Retrying in {:?}... ({}/{})",
                                current_index + 1,
                                total_tokens,
                                args.network_retry_delay,
                                attempt + 1,
                                args.check_retries
                            );
                            sleep(args.network_retry_delay).await;
                        } else {
                            final_result = result;
                            break;
                        }
                    }

                    Err(ref _rate @ ApiError::RateLimited(_)) => {
                        if attempt < args.check_retries - 1 {
                            eprintln!(
                                " -> [Token #{}/{}] Rate Limited (429). Retrying in {:?}... ({}/{})",
                                current_index + 1,
                                total_tokens,
                                args.ratelimit_retry_delay,
                                attempt + 1,
                                args.check_retries
                            );
                            sleep(args.ratelimit_retry_delay).await;
                        } else {
                            final_result = result;
                            break;
                        }
                    }
                    Err(e) => {
                        final_result = Err(e);
                        break;
                    }
                }
            }

            (index, token, final_result)
        });
        tasks.push(task);
    }

    let mut task_outputs: Vec<Result<CheckTaskOutput, JoinError>> =
        Vec::with_capacity(total_tokens);
    for handle in tasks {
        task_outputs.push(handle.await);
    }

    let (mut collected_results, mut join_errors): (Vec<CheckTaskOutput>, Vec<(usize, JoinError)>) =
        task_outputs.into_iter().enumerate().fold(
            (Vec::with_capacity(total_tokens), Vec::new()),
            |(mut successes, mut errors), (original_index, task_result)| {
                match task_result {
                    Ok(output) => successes.push(output),
                    Err(err) => errors.push((original_index, err)),
                }
                (successes, errors)
            },
        );

    collected_results.sort_by_key(|(idx, _, _)| *idx);

    let mut result_counter = 0;
    for (index, original_token, final_result) in collected_results {
        result_counter += 1;
        let mask_flag = args.mask_token;
        let masked_token = if mask_flag {
            Utils::mask_last_part(&original_token)
        } else {
            original_token.clone()
        };

        println!(
            "--- Result #{} (Input Index: {}) [{}] ---",
            result_counter,
            index + 1,
            masked_token
        );

        match final_result {
            Ok(token_info_result) => {
                token_info_result.show(mask_flag);
            }
            Err(e) => {
                eprint!(" -> Error: ");
                match e {
                    ApiError::Unauthorized(resp) => {
                        eprintln!("Token is invalid or expired. {}", resp.message)
                    }
                    ApiError::RateLimited(_) => eprintln!(
                        "Rate Limited (429) after retries on initial check (/users/@me). Cannot proceed."
                    ),
                    ApiError::RequestError(rq_err) => {
                        eprintln!("Network error after retries: {}", rq_err)
                    }
                    ApiError::ParseError(p_err) => {
                        eprintln!("Failed to parse Discord response: {}", p_err)
                    }
                    ApiError::UnexpectedStatus(status, body) => eprintln!(
                        "Discord returned unexpected status {}. Body: {}",
                        status, body
                    ),
                    ApiError::ClientBuildError(build_err) => {
                        eprintln!("Failed to build HTTP client: {}", build_err)
                    }
                    ApiError::IoError(io_err) => eprintln!("I/O error: {}", io_err),
                }
            }
        }
        println!("----------------------------------------\n");
    }

    if !join_errors.is_empty() {
        eprintln!("--- Task Execution Errors ---");
        join_errors.sort_by_key(|(idx, _)| *idx);
        for (original_index, join_err) in join_errors {
            eprintln!(
                " -> Task for input index #{} failed: {}",
                original_index + 1,
                join_err
            );
        }
        eprintln!("-----------------------------\n");
    }

    println!("All checks finished.");
}
