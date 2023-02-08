use std::collections::HashMap;
use std::{path::PathBuf, ffi::OsStr};
use std::str::FromStr;
use std::time::Duration;

#[cfg(feature = "fetch")]
use super::fetcher::{Fetcher, FetcherOptions};
use clap::Parser;
use headless_chrome::{types::PrintToPdfOptions, LaunchOptions};
use humantime::parse_duration;

use crate::Error;

/// Generate a PDF from a local HTML file using a headless chrome
#[derive(Debug, Parser)]
pub struct PdfOptions {
    /// Input HTML file.
    pub input: PathBuf,

    /// Output file.
    /// By default, just change the input extension to PDF
    #[clap(short, long)]
    pub output: Option<PathBuf>,

    /// Use landscape mode.
    #[clap(long)]
    pub landscape: bool,

    /// Allow print background.
    #[clap(long)]
    pub background: bool,

    /// Time to wait in ms before printing.
    /// Examples: 150ms, 10s
    #[clap(long, value_parser = parse_duration)]
    pub wait: Option<Duration>,

    /// HTML template for the print header.
    /// Should be valid HTML markup with following classes used to inject printing values into
    /// them:
    /// date for formatted print date,
    /// title for document title,
    /// url for document location,
    /// pageNumber for current page number,
    /// totalPages for total pages in the document.
    /// For example, `<span class=title></span>` would generate span containing the title.
    #[clap(long)]
    pub header: Option<String>,

    /// HTML template for the print footer.
    /// Should use the same format as the headerTemplate.
    #[clap(long)]
    pub footer: Option<String>,

    /// Paper size.
    /// Supported values: A4, Letter, A3, Tabloid, A2, A1, A0, A5, A6
    #[clap(long)]
    pub paper: Option<PaperSize>,

    /// Scale, default to 1.0
    #[clap(long)]
    pub scale: Option<f64>,

    /// Paper ranges to print,
    /// e.g. '1-5, 8, 11-13'
    #[clap(long)]
    pub range: Option<String>,

    /// Margin in inches
    /// You can define margin like this:
    /// '0.4' the value is applied for all side,
    /// '0.4 0.4' : first value is applied for top and bottom, second for left and right,
    /// '0.4 0.4 0.4 0.4' : first value is applied for top then, right, then bottom, and last for left
    #[clap(long)]
    pub margin: Option<Margin>,
}

impl PdfOptions {
    /// Get a reference to the cli options's input.
    #[must_use]
    pub fn input(&self) -> &PathBuf {
        &self.input
    }

    /// Get a reference to the cli options's output.
    #[must_use]
    pub fn output(&self) -> Option<&PathBuf> {
        self.output.as_ref()
    }

    /// Get a reference to the cli options's landscape.
    #[must_use]
    pub fn landscape(&self) -> bool {
        self.landscape
    }

    /// Get a reference to the cli options's background.
    #[must_use]
    pub fn background(&self) -> bool {
        self.background
    }

    /// Get a reference to the cli options's wait.
    #[must_use]
    pub fn wait(&self) -> Option<Duration> {
        self.wait
    }

    /// Get a reference to the cli options's header.
    #[must_use]
    pub fn header(&self) -> Option<&String> {
        self.header.as_ref()
    }

    /// Get a reference to the cli options's footer.
    #[must_use]
    pub fn footer(&self) -> Option<&String> {
        self.footer.as_ref()
    }

    /// Get a reference to the cli options's paper.
    #[must_use]
    pub fn paper(&self) -> Option<&PaperSize> {
        self.paper.as_ref()
    }

    /// Get a reference to the cli options's scale.
    #[must_use]
    pub fn scale(&self) -> Option<f64> {
        self.scale
    }

    /// Get a reference to the cli options's margin.
    #[must_use]
    pub fn margin(&self) -> Option<&Margin> {
        self.margin.as_ref()
    }

    /// Get a reference to the cli options's range.
    #[must_use]
    pub fn range(&self) -> Option<&String> {
        self.range.as_ref()
    }
}

impl From<&PdfOptions> for PrintToPdfOptions {
    fn from(opt: &PdfOptions) -> Self {
        PrintToPdfOptions {
            landscape: Some(opt.landscape()),
            display_header_footer: Some(opt.header().is_some() || opt.footer().is_some()),
            print_background: Some(opt.background()),
            scale: opt.scale(),
            paper_width: opt.paper().map(PaperSize::paper_width),
            paper_height: opt.paper().map(PaperSize::paper_height),
            margin_top: opt.margin().map(Margin::margin_top),
            margin_bottom: opt.margin().map(Margin::margin_bottom),
            margin_left: opt.margin().map(Margin::margin_left),
            margin_right: opt.margin().map(Margin::margin_right),
            page_ranges: opt.range().cloned(),
            ignore_invalid_page_ranges: None,
            header_template: opt.header().cloned(),
            footer_template: opt.footer().cloned(),
            prefer_css_page_size: None,
            transfer_mode: None,
        }
    }
}

/// Get a reference to the cli launch options's input.
pub struct LocalLaunchOptions<'a> {
    /// Get a reference to the cli options's output.
    pub headless: bool,
    /// Get a reference to the cli options's output.
    pub sandbox: bool,
    /// Get a reference to the cli options's output.
    pub window_size: Option<(u32, u32)>,
    /// Get a reference to the cli options's output.
    pub port: Option<u16>,
    /// Get a reference to the cli options's output.
    pub ignore_certificate_errors: bool,
    /// Get a reference to the cli options's output.
    pub path: Option<std::path::PathBuf>,
    /// Get a reference to the cli options's output.
    pub user_data_dir: Option<std::path::PathBuf>,
    /// Get a reference to the cli options's output.
    pub extensions: Vec<&'a OsStr>,
    /// Get a reference to the cli options's output.
    pub args: Vec<&'a OsStr>,
    /// Get a reference to the cli options's output.
    pub disable_default_args: bool,
    /// Get a reference to the cli options's output.
    pub idle_browser_timeout: Duration,
    /// Get a reference to the cli options's output.
    pub process_envs: Option<HashMap<String, String>>,
    /// Get a reference to the cli options's output.
    pub proxy_server: Option<&'a str>,
}

impl<'a> LocalLaunchOptions<'a> {
    /// Get a reference to the cli options's input.
    #[must_use]
    pub fn headless(&self) -> bool {
        self.headless
    }

    /// Get a reference to the cli options's output.
    #[must_use]
    pub fn sandbox(&self) -> bool {
        self.sandbox
    }

    /// Get a reference to the cli options's landscape.
    #[must_use]
    pub fn window_size(&self) -> Option<(u32, u32)> {
        self.window_size
    }

    /// Get a reference to the cli options's background.
    #[must_use]
    pub fn port(&self) -> Option<u16> {
        self.port
    }

    /// Get a reference to the cli options's wait.
    #[must_use]
    pub fn ignore_certificate_errors(&self) -> bool {
        self.ignore_certificate_errors
    }

    /// Get a reference to the cli options's header.
    #[must_use]
    pub fn path(&self) -> Option<std::path::PathBuf> {
        self.path.clone()
    }

    /// Get a reference to the cli options's footer.
    #[must_use]
    pub fn user_data_dir(&self) -> Option<std::path::PathBuf> {
        self.user_data_dir.clone()
    }

    /// Get a reference to the cli options's paper.
    #[must_use]
    pub fn extension(&self) -> Vec<&'a OsStr> {
        self.extensions.clone()
    }

    /// Get a reference to the cli options's scale.
    #[must_use]
    pub fn args(&self) -> Vec<&'a OsStr> {
        self.args.clone()
    }

    /// Get a reference to the cli options's margin.
    #[must_use]
    pub fn disable_default_args(&self) -> bool {
        self.disable_default_args
    }

    /// Get a reference to the cli options's range.
    #[must_use]
    pub fn idle_browser_timeout(&self) -> Duration {
        self.idle_browser_timeout
    }

    /// Get a reference to the cli options's range.
    #[must_use]
    pub fn process_envs(&self) -> Option<HashMap<String, String>> {
        self.process_envs.clone()
    }

    /// Get a reference to the cli options's range.
    #[must_use]
    pub fn proxy_server(&self) -> Option<&'a str> {
        self.proxy_server
    }
}

impl<'a> From<&LocalLaunchOptions<'a>> for LaunchOptions<'a> {
    fn from(local_launch_options: &LocalLaunchOptions<'a>) -> Self {
        LaunchOptions::<'a> {
            headless: local_launch_options.headless,
            sandbox: local_launch_options.sandbox,
            window_size: local_launch_options.window_size,
            port: local_launch_options.port,
            ignore_certificate_errors: local_launch_options.ignore_certificate_errors,
            path: local_launch_options.path.clone(),
            user_data_dir: local_launch_options.user_data_dir.clone(),
            extensions: local_launch_options.extensions.clone(),
            args: local_launch_options.args.clone(),
            disable_default_args: local_launch_options.disable_default_args,
            idle_browser_timeout: local_launch_options.idle_browser_timeout,
            process_envs: local_launch_options.process_envs.clone(),
            proxy_server: local_launch_options.proxy_server,
        }
    }
}

/// Paper size
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PaperSize {
    /// A0 (84.1cm × 118.9cm)
    A0,
    /// A1 (59.4cm × 84.1cm)
    A1,
    /// A2 (42.0cm × 59.4cm)
    A2,
    /// A3 (29.7cm × 42.0cm)
    A3,
    /// A4 (21.0cm × 29.7 cm)
    A4,
    /// A5 (14.8cm × 21.0cm)
    A5,
    /// A6 (10.5cm × 14.8cm)
    A6,
    /// US Letter (11.0in × 8.5in)
    Letter,
    /// Legal (17in × 8.5in)
    Legal,
    /// Tabloid (17in × 11in)
    Tabloid,
}

impl PaperSize {
    /// The width
    #[must_use]
    pub fn paper_width(&self) -> f64 {
        match self {
            PaperSize::A0 => 33.1,
            PaperSize::A1 => 23.4,
            PaperSize::A2 => 16.5,
            PaperSize::A3 => 11.7,
            PaperSize::A4 => 8.27,
            PaperSize::A5 => 5.83,
            PaperSize::A6 => 4.13,
            PaperSize::Letter | PaperSize::Legal => 8.5,
            PaperSize::Tabloid => 11.0,
        }
    }

    /// The height
    #[must_use]
    pub fn paper_height(&self) -> f64 {
        match self {
            PaperSize::A0 => 46.8,
            PaperSize::A1 => 33.1,
            PaperSize::A2 => 23.4,
            PaperSize::A3 => 16.5,
            PaperSize::A4 => 11.7,
            PaperSize::A5 => 8.27,
            PaperSize::A6 => 5.83,
            PaperSize::Letter => 11.0,
            PaperSize::Legal | PaperSize::Tabloid => 17.0,
        }
    }
}

impl FromStr for PaperSize {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "a0" => Ok(Self::A0),
            "a1" => Ok(Self::A1),
            "a2" => Ok(Self::A2),
            "a3" => Ok(Self::A3),
            "a4" => Ok(Self::A4),
            "a5" => Ok(Self::A5),
            "a6" => Ok(Self::A6),
            "letter" => Ok(Self::Letter),
            "legal" => Ok(Self::Legal),
            "tabloid" => Ok(Self::Tabloid),
            _ => Err(Error::InvalidPaperSize(s.to_string())),
        }
    }
}

/// Margin definition
#[derive(Debug, Clone, PartialEq)]
pub enum Margin {
    /// Same margin on all side
    All(f64),
    /// Same margin vertically and horizontally
    VerticalHorizontal(f64, f64),
    /// Custom margin for every side
    TopRightBottomLeft(f64, f64, f64, f64),
}

impl Margin {
    /// Margin top
    #[must_use]
    pub fn margin_top(&self) -> f64 {
        match self {
            Margin::All(f)
            | Margin::VerticalHorizontal(f, _)
            | Margin::TopRightBottomLeft(f, _, _, _) => *f,
        }
    }
    /// Margin right
    #[must_use]
    pub fn margin_right(&self) -> f64 {
        match self {
            Margin::All(f)
            | Margin::VerticalHorizontal(_, f)
            | Margin::TopRightBottomLeft(_, f, _, _) => *f,
        }
    }
    /// Margin bottom
    #[must_use]
    pub fn margin_bottom(&self) -> f64 {
        match self {
            Margin::All(f)
            | Margin::VerticalHorizontal(f, _)
            | Margin::TopRightBottomLeft(_, _, f, _) => *f,
        }
    }
    /// Margin left
    #[must_use]
    pub fn margin_left(&self) -> f64 {
        match self {
            Margin::All(f)
            | Margin::VerticalHorizontal(_, f)
            | Margin::TopRightBottomLeft(_, _, _, f) => *f,
        }
    }
}

impl FromStr for Margin {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values: Vec<&str> = s.split(' ').filter(|s| !s.is_empty()).collect();
        match values.len() {
            1 => {
                let value = s.parse::<f64>().map_err(Error::InvalidMarginValue)?;
                Ok(Margin::All(value))
            }
            2 => {
                let v = values[0]
                    .parse::<f64>()
                    .map_err(Error::InvalidMarginValue)?;
                let h = values[1]
                    .parse::<f64>()
                    .map_err(Error::InvalidMarginValue)?;
                Ok(Margin::VerticalHorizontal(v, h))
            }
            4 => {
                let top = values[0]
                    .parse::<f64>()
                    .map_err(Error::InvalidMarginValue)?;
                let right = values[1]
                    .parse::<f64>()
                    .map_err(Error::InvalidMarginValue)?;
                let bottom = values[2]
                    .parse::<f64>()
                    .map_err(Error::InvalidMarginValue)?;
                let left = values[2]
                    .parse::<f64>()
                    .map_err(Error::InvalidMarginValue)?;
                Ok(Margin::TopRightBottomLeft(top, right, bottom, left))
            }
            _ => Err(Error::InvalidMarginDefinition(s.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use assert2::{check, let_assert};
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::a0("a0", PaperSize::A0)]
    #[case::a1("A1", PaperSize::A1)]
    #[case::a2("A2", PaperSize::A2)]
    #[case::a3("A3", PaperSize::A3)]
    #[case::a4("A4", PaperSize::A4)]
    #[case::a5("A5", PaperSize::A5)]
    #[case::a6("A6", PaperSize::A6)]
    #[case::letter("letter", PaperSize::Letter)]
    #[case::legal("Legal", PaperSize::Legal)]
    #[case::tabloid("Tabloid", PaperSize::Tabloid)]
    fn should_parse_valid_paper_size(#[case] value: &str, #[case] expected: PaperSize) {
        let result = value.parse::<PaperSize>().unwrap();
        check!(result == expected);
    }

    #[test]
    fn should_reject_invalid_paper_size() {
        let value = "plop";
        let result = value.parse::<PaperSize>();
        let_assert!(Err(Error::InvalidPaperSize(_)) = result);
    }

    #[test]
    fn should_parse_valid_margin_all() {
        let value = "0.4";
        let result = value.parse::<Margin>();
        let_assert!(Ok(Margin::All(_)) = result);
    }

    #[test]
    fn should_parse_valid_margin_vh() {
        let value = "0.4  0.7";
        let result = value.parse::<Margin>();
        let_assert!(Ok(Margin::VerticalHorizontal(_, _)) = result);
    }

    #[test]
    fn should_parse_valid_margin_trbl() {
        let value = "0.2   0.3 0.4  0.5";
        let result = value.parse::<Margin>();
        let_assert!(Ok(Margin::TopRightBottomLeft(_, _, _, _)) = result);
    }

    #[test]
    fn should_reject_invalid_margin() {
        let value = "0.2    0.3  0.4";
        let result = value.parse::<Margin>();
        let_assert!(Err(Error::InvalidMarginDefinition(_)) = result);
    }

    #[test]
    fn should_reject_invalid_margin_value() {
        let value = "plop";
        let result = value.parse::<Margin>();
        let_assert!(Err(Error::InvalidMarginValue(_)) = result);
    }
}
