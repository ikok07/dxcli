use chrono::{DateTime, Duration};
use chrono_tz::{Tz, TZ_VARIANTS};
use md5::digest::typenum::op;
use crate::cli::{TimeAgoOptions, TimeFormatOptions, TimeFromUnixOptions, TimeMethod, TimeNowOptions, TimeToUnixOptions};
use crate::handlers::{Result, CommandHandlerError};

pub struct TimeHandler {}

impl TimeHandler {
    pub fn handle_method(method: &TimeMethod) -> Result {
        match method {
            TimeMethod::Now {options} => Self::get_current_datetime(options),
            TimeMethod::Unix => Ok(chrono::Utc::now().timestamp().to_string()),
            TimeMethod::FromUnix {options} => Self::convert_from_unix(options),
            TimeMethod::ToUnix {options} => Self::convert_to_unix(options),
            TimeMethod::Relative {options} => Self::calculate_relative_time(options),
            TimeMethod::Format {options} => Self::format_date(options),
            TimeMethod::Tz => Ok(TZ_VARIANTS.iter().map(|tz| format!("- {tz}")).collect::<Vec<String>>().join("\n"))
        }
    }

    fn get_current_datetime(options: &TimeNowOptions) -> Result {
        let format = "%d/%m/%Y %H:%M:%S %Z";
        if options.timezone.is_some() {
            let timezone = options.timezone.as_ref().unwrap();
            let tz: Tz = timezone.parse().map_err(|_| {
                return CommandHandlerError::RuntimeError(Some(Self::invalid_tz_message(timezone)))
            })?;
            return Ok(chrono::Utc::now().with_timezone(&tz).format(format).to_string());
        }

        return Ok(chrono::Local::now().format(format).to_string())
    }

    fn convert_from_unix(options: &TimeFromUnixOptions) -> Result {
        match chrono::DateTime::from_timestamp(options.timestamp, 0) {
            Some(date) => {
                if options.timezone.is_some() {
                    let timezone = options.timezone.as_ref().unwrap();
                    let tz: Tz = timezone.parse().map_err(|_| {
                        return CommandHandlerError::RuntimeError(Some(Self::invalid_tz_message(timezone)))
                    })?;
                    return Ok(date.with_timezone(&tz).to_rfc3339());
                }

                return Ok(date.to_rfc3339());
            },
            None => Err(CommandHandlerError::RuntimeError(Some(format!("Timestamp '{}' is invalid!", options.timestamp))))
        }
    }

    fn convert_to_unix(options: &TimeToUnixOptions) -> Result {
        let err_convert = |_| CommandHandlerError::RuntimeError(Some(format!("Failed to parse date '{}'!", options.date)));

        // Check if timezone is included
        if Self::has_format_timezone(&options.format) {
            let date = chrono::DateTime::parse_from_str(&options.date, &options.format)
                .map_err(err_convert)?;

            return Ok(date.timestamp().to_string());
        } else if Self::has_format_time(&options.format) {
            let date = chrono::NaiveDateTime::parse_from_str(&options.date, &options.format)
                .map_err(err_convert)?;

            return Ok(date.and_utc().timestamp().to_string());
        } else {
            let date = chrono::NaiveDate::parse_from_str(&options.date, &options.format)
                .map_err(err_convert)?;

            return Ok(date.and_hms_opt(0, 0, 0).unwrap().and_utc().timestamp().to_string());
        }
    }

    fn calculate_relative_time(options: &TimeAgoOptions) -> Result {
        match chrono::DateTime::from_timestamp(options.timestamp, 0) {
            Some(date) => {
                let start_date = {
                    if options.start.is_some() && let Some(parsed_start_date) = DateTime::from_timestamp(options.start.unwrap(), 0) {
                        parsed_start_date
                    } else {
                        chrono::Utc::now()
                    }
                };
                let duration = start_date - date;
                let mut response = String::new();
                let mut seconds = duration.num_seconds().abs();

                let weeks = seconds / Duration::weeks(1).num_seconds();
                seconds %= Duration::weeks(1).num_seconds();

                let days = seconds / Duration::days(1).num_seconds();
                seconds %= Duration::days(1).num_seconds();

                let hours = seconds / Duration::hours(1).num_seconds();
                seconds %= Duration::hours(1).num_seconds();

                let minutes = seconds / Duration::minutes(1).num_seconds();
                seconds %= Duration::minutes(1).num_seconds();

                if weeks > 0 {
                    response.push_str(format!("{} Weeks ", weeks).as_str());
                };
                if days > 0 {
                    response.push_str(format!("{} Days ", days).as_str());
                }
                if hours > 0 {
                    response.push_str(format!("{} Hours ", hours).as_str());
                };
                if minutes > 0 {
                    response.push_str(format!("{} Minutes ", minutes).as_str());
                };

                response.push_str(format!("{} Seconds", seconds).as_str());

                return Ok(response);
            },
            None => Err(CommandHandlerError::RuntimeError(Some(format!("Timestamp '{}' is invalid!", options.timestamp))))
        }
    }

    fn format_date(options: &TimeFormatOptions) -> Result {
        let err_convert = |_| CommandHandlerError::RuntimeError(Some(format!("Failed to parse date '{}'!", options.date)));
        if Self::has_format_timezone(&options.from) {
            let date = chrono::DateTime::parse_from_str(&options.date, &options.from)
                .map_err(err_convert)?;

            return Ok(date.format(&options.target).to_string());
        } else if Self::has_format_time(&options.from) {
            let date = chrono::NaiveDateTime::parse_from_str(&options.date, &options.from)
                .map_err(err_convert)?;

            return Ok(date.format(&options.target).to_string());
        } else {
            let date = chrono::NaiveDate::parse_from_str(&options.date, &options.from)
                .map_err(err_convert)?;

            return Ok(date.format(&options.target).to_string());
        }
    }

    fn invalid_tz_message(timezone: &str) -> String {
        format!("Timezone '{}' is invalid! Run 'dx time tz' to list all available timezones", timezone)
    }

    fn has_format_timezone(format: &str) -> bool {
        return ["%Z", "%z"].iter().any(|pattern| format.contains(pattern));
    }

    fn has_format_time(format: &str) -> bool {
        return format.contains("%H") || format.contains("%M") ||
            format.contains("%S") || format.contains("%T");
    }
}

#[cfg(test)]
mod tests {
    use chrono::{DateTime, Utc};
    use super::*;
    use pretty_assertions::{assert_eq};

    #[test]
    fn check_for_correct_conversion_from_unix() {
        let timestamp = 1767352174;
        let result = TimeHandler::convert_from_unix(&TimeFromUnixOptions {
            timestamp,
            timezone: Some(Tz::Europe__Sofia.to_string())
        });

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), DateTime::from_timestamp(timestamp, 0).unwrap().with_timezone(&Tz::Europe__Sofia).to_rfc3339());
    }

    #[test]
    fn check_for_correct_conversion_to_unix() {
        let result = TimeHandler::convert_to_unix(&TimeToUnixOptions{
            date: "02/01/2026 00:00 +0000".to_string(),
            format: "%d/%m/%Y %H:%M %z".to_string(),
        });
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), String::from("1767312000"));
    }

    #[test]
    fn check_for_correct_relative_time_calculation() {
        let start_timestamp = 1767139200; // 31.12.2025 00:00:00 UTC
        let compare_timestamp = 1764592225; // 01.12.2025 12:30:25 UTC
        let result = TimeHandler::calculate_relative_time(&TimeAgoOptions {
            timestamp: compare_timestamp,
            start: Some(start_timestamp)
        });

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), String::from("4 Weeks 1 Days 11 Hours 29 Minutes 35 Seconds"));
    }

    #[test]
    fn check_for_correct_date_reformatting() {
        let result = TimeHandler::format_date(&TimeFormatOptions {
            date: "02/01/2026 00:00 +0200".to_string(),
            from: "%d/%m/%Y %H:%M %z".to_string(),
            target: "%d.%m.%Y %H:%M (%z)".to_string(),
        });

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "02.01.2026 00:00 (+0200)");
    }
}