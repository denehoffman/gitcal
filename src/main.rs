use clap::{arg, ArgGroup, Command};
use colored::{Color, ColoredString, Colorize};
use hex_color::{HexColor, ParseHexColorError};
use jiff::{ToSpan, Zoned};
use reqwest::{self, blocking::Client};
use serde_json::Value;
use std::{error::Error, fmt::Display};

const GITHUB_API_URL: &str = "https://api.github.com/graphql";
struct Palette {
    text: Color,
    base: Color,
    color0: Color,
    color1: Color,
    color2: Color,
    color3: Color,
    color4: Color,
}
impl Default for Palette {
    fn default() -> Self {
        Self {
            text: Color::White,
            base: Color::TrueColor {
                r: 14,
                g: 17,
                b: 33,
            },
            color0: Color::TrueColor {
                r: 23,
                g: 27,
                b: 33,
            },
            color1: Color::TrueColor {
                r: 31,
                g: 67,
                b: 43,
            },
            color2: Color::TrueColor {
                r: 46,
                g: 108,
                b: 56,
            },
            color3: Color::TrueColor {
                r: 81,
                g: 163,
                b: 78,
            },
            color4: Color::TrueColor {
                r: 108,
                g: 208,
                b: 99,
            },
        }
    }
}
impl Palette {
    fn with_text(mut self, hex: String) -> Result<Self, ParseHexColorError> {
        let color = HexColor::parse(&hex)?;
        self.text = Color::TrueColor {
            r: color.r,
            g: color.g,
            b: color.b,
        };
        Ok(self)
    }
    fn with_base(mut self, hex: String) -> Result<Self, ParseHexColorError> {
        let color = HexColor::parse(&hex)?;
        self.base = Color::TrueColor {
            r: color.r,
            g: color.g,
            b: color.b,
        };
        Ok(self)
    }
    fn with_color0(mut self, hex: String) -> Result<Self, ParseHexColorError> {
        let color = HexColor::parse(&hex)?;
        self.color0 = Color::TrueColor {
            r: color.r,
            g: color.g,
            b: color.b,
        };
        Ok(self)
    }
    fn with_color1(mut self, hex: String) -> Result<Self, ParseHexColorError> {
        let color = HexColor::parse(&hex)?;
        self.color1 = Color::TrueColor {
            r: color.r,
            g: color.g,
            b: color.b,
        };
        Ok(self)
    }
    fn with_color2(mut self, hex: String) -> Result<Self, ParseHexColorError> {
        let color = HexColor::parse(&hex)?;
        self.color2 = Color::TrueColor {
            r: color.r,
            g: color.g,
            b: color.b,
        };
        Ok(self)
    }
    fn with_color3(mut self, hex: String) -> Result<Self, ParseHexColorError> {
        let color = HexColor::parse(&hex)?;
        self.color3 = Color::TrueColor {
            r: color.r,
            g: color.g,
            b: color.b,
        };
        Ok(self)
    }
    fn with_color4(mut self, hex: String) -> Result<Self, ParseHexColorError> {
        let color = HexColor::parse(&hex)?;
        self.color4 = Color::TrueColor {
            r: color.r,
            g: color.g,
            b: color.b,
        };
        Ok(self)
    }
}
#[derive(Copy, Clone, Default)]
enum DisplayStyle {
    #[default]
    SmallSquare,
    FullBlock,
    HalfBlock,
    Circle,
}
impl DisplayStyle {
    fn get_tile(&self) -> String {
        match self {
            Self::SmallSquare => " ■",
            Self::FullBlock => " █",
            Self::HalfBlock => "█",
            Self::Circle => "",
        }
        .to_string()
    }
    fn get_tile_size(&self) -> usize {
        match self {
            Self::SmallSquare | Self::FullBlock | Self::Circle => 2,
            Self::HalfBlock => 1,
        }
    }
}
struct GithubMonth(String, usize);
#[derive(Default, Copy, Clone, Debug)]
enum GithubQuartiles {
    First,
    Second,
    Third,
    Fourth,
    #[default]
    None,
}
impl GithubQuartiles {
    fn get_color(&self, palette: &Palette) -> Color {
        match self {
            GithubQuartiles::First => palette.color1,
            GithubQuartiles::Second => palette.color2,
            GithubQuartiles::Third => palette.color3,
            GithubQuartiles::Fourth => palette.color4,
            GithubQuartiles::None => palette.color0,
        }
    }
    fn get_tile(&self, style: DisplayStyle, palette: &Palette) -> ColoredString {
        style
            .get_tile()
            .color(self.get_color(palette))
            .on_color(palette.base)
    }
}
impl<'a> From<&'a str> for GithubQuartiles {
    fn from(value: &'a str) -> Self {
        match value {
            "FIRST_QUARTILE" => Self::First,
            "SECOND_QUARTILE" => Self::Second,
            "THIRD_QUARTILE" => Self::Third,
            "FOURTH_QUARTILE" => Self::Fourth,
            "NONE" => Self::None,
            _ => unreachable!(),
        }
    }
}

#[derive(Default)]
struct Calendar {
    data: Vec<Vec<GithubQuartiles>>,
    months: Vec<GithubMonth>,
    palette: Palette,
    style: DisplayStyle,
    show_days: bool,
    show_months: bool,
}

impl Calendar {
    fn with_data(mut self, data: Vec<Vec<GithubQuartiles>>) -> Self {
        self.data = data;
        self
    }
    fn with_months(mut self, months: Vec<GithubMonth>) -> Self {
        self.months = months;
        self
    }
    fn with_palette(mut self, palette: Palette) -> Self {
        self.palette = palette;
        self
    }
    fn with_style(mut self, style: DisplayStyle) -> Self {
        self.style = style;
        self
    }
    fn with_show_days(mut self, show_days: bool) -> Self {
        self.show_days = show_days;
        self
    }
    fn with_show_months(mut self, show_months: bool) -> Self {
        self.show_months = show_months;
        self
    }
}
impl Display for Calendar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let endcap = matches!(self.style, DisplayStyle::Circle);
        if self.show_months {
            if self.show_days {
                write!(f, "{}", "    ".on_color(self.palette.base))?;
            }
            write!(f, "{}", " ".on_color(self.palette.base))?;
            for month in &self.months {
                if (month.1 * self.style.get_tile_size()) > month.0.len() {
                    // only write if there's space
                    write!(
                        f,
                        "{}",
                        month.0.color(self.palette.text).on_color(self.palette.base)
                    )?;
                    write!(
                        f,
                        "{:1$}",
                        "".on_color(self.palette.base),
                        (month.1 * self.style.get_tile_size()) - month.0.len()
                    )?;
                } else {
                    write!(
                        f,
                        "{:1$}",
                        "".on_color(self.palette.base),
                        (month.1 * self.style.get_tile_size())
                    )?;
                }
            }
            if endcap {
                write!(f, "{}", " ".on_color(self.palette.base))?;
            }
            writeln!(f)?;
        }
        for (i, day_of_week) in self.data.iter().enumerate() {
            if self.show_days {
                match i {
                    1 => write!(
                        f,
                        "{}",
                        " Mon".color(self.palette.text).on_color(self.palette.base)
                    )?,
                    3 => write!(
                        f,
                        "{}",
                        " Wed".color(self.palette.text).on_color(self.palette.base)
                    )?,
                    5 => write!(
                        f,
                        "{}",
                        " Fri".color(self.palette.text).on_color(self.palette.base)
                    )?,
                    _ => write!(f, "{}", "    ".on_color(self.palette.base))?,
                }
            }
            if endcap {
                write!(f, "{}", " ".on_color(self.palette.base))?;
            }
            for week in day_of_week.iter() {
                write!(f, "{}", week.get_tile(self.style, &self.palette))?;
            }
            writeln!(f, "{}", " ".on_color(self.palette.base))?;
        }

        if self.show_months {
            writeln!(
                f,
                "{:1$}",
                "".on_color(self.palette.base),
                self.data[0].len() * self.style.get_tile_size()
                    + if self.show_days { 5 } else { 1 }
                    + if endcap { 1 } else { 0 }
            )?;
        }
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let matches = Command::new("gitcal")
        .about("A CLI tool for calendar visualization")
        .arg(arg!(--username <NAME> "GitHub username (defaults to token owner)"))
        .arg(arg!(--block "uUe block icons"))
        .arg(arg!(--half "Use block icons without spaces"))
        .arg(arg!(--circle "Use circle icons"))
        .group(
            ArgGroup::new("shape")
                .args(["block", "circle", "half"])
                .multiple(false)
                .required(false),
        )
        .arg(arg!(--base <HEX> "Set base color"))
        .arg(arg!(--text <HEX> "Set text color"))
        .arg(arg!(--color0 <HEX> "Set color for no contributions"))
        .arg(arg!(--color1 <HEX> "Set color for first quartile"))
        .arg(arg!(--color2 <HEX> "Set color for second quartile"))
        .arg(arg!(--color3 <HEX> "Set color for third quartile"))
        .arg(arg!(--color4 <HEX> "Set color for fourth quartile"))
        .arg(arg!(--ytd "Display the past year's worth of data"))
        .arg(arg!(--month "Display the past month's worth of data"))
        .group(
            ArgGroup::new("time")
                .args(["ytd", "month"])
                .multiple(false)
                .required(false),
        )
        .arg(arg!(--token <GITHUB_TOKEN> "GitHub PAT token (uses $GITHUB_TOKEN if not specified)"))
        .arg(arg!(--"hide-days" "Hide day-of-the-week string"))
        .arg(arg!(--"hide-months" "Hide months in header"))
        .get_matches();
    let username = matches.get_one::<String>("username");
    let token = if let Some(tkn) = matches.get_one::<String>("token") {
        tkn.to_owned()
    } else {
        std::env::var("GITHUB_TOKEN").unwrap_or_else(|_| {
            eprintln!("Set $GITHUB_TOKEN or use the --token argument!");
            "".to_string()
        })
    };
    if token.is_empty() {
        return Ok(());
    }
    let style = if matches.get_flag("block") {
        DisplayStyle::FullBlock
    } else if matches.get_flag("circle") {
        DisplayStyle::Circle
    } else if matches.get_flag("half") {
        DisplayStyle::HalfBlock
    } else {
        DisplayStyle::SmallSquare
    };
    let mut palette = Palette::default();
    if let Some(hex) = matches.get_one::<String>("base") {
        palette = palette.with_base(hex.to_owned())?;
    }
    if let Some(hex) = matches.get_one::<String>("text") {
        palette = palette.with_text(hex.to_owned())?;
    }
    if let Some(hex) = matches.get_one::<String>("color0") {
        palette = palette.with_color0(hex.to_owned())?;
    }
    if let Some(hex) = matches.get_one::<String>("color1") {
        palette = palette.with_color1(hex.to_owned())?;
    }
    if let Some(hex) = matches.get_one::<String>("color2") {
        palette = palette.with_color2(hex.to_owned())?;
    }
    if let Some(hex) = matches.get_one::<String>("color3") {
        palette = palette.with_color3(hex.to_owned())?;
    }
    if let Some(hex) = matches.get_one::<String>("color4") {
        palette = palette.with_color4(hex.to_owned())?;
    }

    let now = Zoned::now();
    let time_start = if matches.get_flag("ytd") {
        now.first_of_year()?
    } else if matches.get_flag("month") {
        now.first_of_month()?
    } else {
        now.checked_sub(1.year())?
    };

    let client = Client::new();
    let query = format!(
        r#"
        query {{
          {} {{
            contributionsCollection(from: "{}", to: "{}") {{
              contributionCalendar {{
                weeks {{
                    contributionDays {{ 
                        weekday
                        contributionLevel
                    }}
                }}
                months {{
                    name
                    totalWeeks
                }}
              }}
            }}
          }}
        }}
        "#,
        if let Some(name) = username {
            format!(r#"user(login: "{}")"#, name)
        } else {
            "viewer".to_string()
        },
        time_start.datetime(),
        now.datetime()
    );

    let value = serde_json::json!({ "query": query });
    let resp: Value = serde_json::from_str(
        &client
            .post(GITHUB_API_URL)
            .header("Accept", "application/json")
            .header("User-Agent", "Rust")
            .bearer_auth(token)
            .body(value.to_string())
            .send()?
            .text()?,
    )?;
    let cal_data = &resp["data"][if username.is_none() { "viewer" } else { "user" }]
        ["contributionsCollection"]["contributionCalendar"];
    let weeks = cal_data["weeks"].as_array().unwrap();
    let mut data: Vec<Vec<GithubQuartiles>> =
        vec![vec![GithubQuartiles::default(); weeks.len()]; 7];
    for (week_index, week) in weeks.iter().enumerate() {
        for day in week["contributionDays"].as_array().unwrap().iter() {
            let quartile_str = day["contributionLevel"].as_str().unwrap();
            let day_index = day["weekday"].as_u64().unwrap() as usize;
            data[day_index][week_index] = GithubQuartiles::from(quartile_str);
        }
    }
    let month_data: Vec<GithubMonth> = cal_data["months"]
        .as_array()
        .unwrap()
        .iter()
        .map(|month| {
            GithubMonth(
                month["name"].as_str().unwrap().to_string(),
                month["totalWeeks"].as_u64().unwrap() as usize,
            )
        })
        .collect();
    let calendar = Calendar::default()
        .with_data(data)
        .with_months(month_data)
        .with_style(style)
        .with_palette(palette)
        .with_show_days(!matches.get_flag("hide-days"))
        .with_show_months(!matches.get_flag("hide-months"));
    println!("{}", calendar);
    Ok(())
}
