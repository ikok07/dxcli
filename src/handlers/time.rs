use chrono::{DateTime, Duration, TimeZone, Utc};
use chrono_tz::{Tz, TZ_VARIANTS};
use crate::cli::{TimeAgoOptions, TimeFormatOptions, TimeFromUnixOptions, TimeMethod, TimeNowOptions, TimeToUnixOptions};
use crate::handlers::CommandHandlerError;

pub struct TimeHandler {}

impl TimeHandler {
    pub fn handle_method(method: &TimeMethod) -> Result<String, CommandHandlerError> {
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

    fn get_current_datetime(options: &TimeNowOptions) -> Result<String, CommandHandlerError> {
        let format = "%d/%m/%Y %H:%M:%S %Z";
        if options.timezone.is_some() {
            let timezone = options.timezone.as_ref().unwrap();
            let tz: Tz = timezone.parse().map_err(|err| {
                return CommandHandlerError::RuntimeError(Some(Self::invalid_tz_message(timezone)))
            })?;
            return Ok(chrono::Utc::now().with_timezone(&tz).format(format).to_string());
        }

        return Ok(chrono::Local::now().format(format).to_string())
    }

    fn convert_from_unix(options: &TimeFromUnixOptions) -> Result<String, CommandHandlerError> {
        match chrono::DateTime::from_timestamp(options.timestamp, 0) {
            Some(date) => {
                if options.timezone.is_some() {
                    let timezone = options.timezone.as_ref().unwrap();
                    let tz: Tz = timezone.parse().map_err(|err| {
                        return CommandHandlerError::RuntimeError(Some(Self::invalid_tz_message(timezone)))
                    })?;
                    return Ok(date.with_timezone(&tz).to_rfc3339());
                }

                return Ok(date.to_rfc3339());
            },
            None => Err(CommandHandlerError::RuntimeError(Some(format!("Timestamp '{}' is invalid!", options.timestamp))))
        }
    }

    fn convert_to_unix(options: &TimeToUnixOptions) -> Result<String, CommandHandlerError> {
        let err_convert = |err| CommandHandlerError::RuntimeError(Some(format!("Failed to parse date '{}'!", options.date)));

        // Check if timezone is included
        if Self::has_format_timezone(&options.format) {
            let date = chrono::DateTime::parse_from_str(&options.date, &options.format)
                .map_err(err_convert)?;

            return Ok(date.timestamp().to_string());
        } else {
            let date = chrono::NaiveDateTime::parse_from_str(&options.date, &options.format)
                .map_err(err_convert)?;

            return Ok(date.and_utc().timestamp().to_string());
        }
    }

    fn calculate_relative_time(options: &TimeAgoOptions) -> Result<String, CommandHandlerError> {
        match chrono::DateTime::from_timestamp(options.timestamp, 0) {
            Some(date) => {
                let duration = chrono::Utc::now() - date;
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
    
    fn format_date(options: &TimeFormatOptions) -> Result<String, CommandHandlerError> {
        let err_convert = |err| CommandHandlerError::RuntimeError(Some(format!("Failed to parse date '{}'!", options.date)));
        if Self::has_format_timezone(&options.from) {
            let date = chrono::DateTime::parse_from_str(&options.date, &options.from)
                .map_err(err_convert)?;

            return Ok(date.format(&options.target).to_string());
        } else {
            let date = chrono::NaiveDateTime::parse_from_str(&options.date, &options.from)
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
}