use clap::{Args, Subcommand};

#[derive(Debug, Subcommand)]
pub enum TimeMethod {
    #[command(about = "Get current date and time")]
    Now {
        #[command(flatten)]
        options: TimeNowOptions
    },
    #[command(about = "Get Unix timestamp")]
    Unix,
    #[command(about = "Convert from Unix timestamp")]
    FromUnix {
        #[command(flatten)]
        options: TimeFromUnixOptions
    },
    #[command(about = "Convert to Unix timestamp")]
    ToUnix {
        #[command(flatten)]
        options: TimeToUnixOptions
    },
    #[command(about = "Calculate relative time")]
    Relative {
        #[command(flatten)]
        options: TimeAgoOptions
    },
    #[command(about = "Reformat date")]
    Format {
        #[command(flatten)]
        options: TimeFormatOptions
    },
    #[command(about = "List all available timezones")]
    Tz
}

#[derive(Debug, Args)]
pub struct TimeNowOptions {
    #[arg(long, short, required = false, help = "Timezone")]
    pub timezone: Option<String>
}

#[derive(Debug, Args)]
pub struct TimeFromUnixOptions {
    #[arg(required = true, help = "Unix Timestamp")]
    pub timestamp: i64,

    #[arg(long, short, required = false, help = "Timezone")]
    pub timezone: Option<String>
}

#[derive(Debug, Args)]
pub struct TimeToUnixOptions {
    #[arg(required = true, help = "Date string")]
    pub date: String,

    #[arg(long, short, required = true, help = "strftime/strptime date format")]
    pub format: String
}

#[derive(Debug, Args)]
pub struct TimeAgoOptions {
    #[arg(required = true, help = "Unix Timestamp")]
    pub timestamp: i64,

    #[arg(long, short, required = false, help = "Start timestamp")]
    pub start: Option<i64>
}

#[derive(Debug, Args)]
pub struct TimeFormatOptions {
    #[arg(required = true, help = "Date string")]
    pub date: String,

    #[arg(long, short, required = true, help = "Original date format (strftime/strptime)")]
    pub from: String,

    #[arg(long, short, required = true, help = "Target date format (strftime/strptime)")]
    pub target: String
}