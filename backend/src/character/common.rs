use serde::{Deserialize, Serialize};
use sqlx::{Row, sqlite::SqliteRow};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Element {
    WIND = 1,
    FOREST = 2,
    FIRE = 3,
    MOUNTAIN = 4,
    UNKNOWN = 5,
}

impl From<i32> for Element {
    fn from(value: i32) -> Self {
        match value {
            1 => Element::WIND,
            2 => Element::FOREST,
            3 => Element::FIRE,
            4 => Element::MOUNTAIN,
            _ => Element::UNKNOWN,
        }
    }
}

#[repr(u8)]
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Position {
    GK = 1,
    DF = 4,
    MF = 3,
    FW = 2,
    UNKNOWN = 5,
}

impl From<i32> for Position {
    fn from(value: i32) -> Self {
        match value {
            1 => Position::GK,
            2 => Position::FW,
            3 => Position::MF,
            4 => Position::DF,
            _ => Position::UNKNOWN,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Style {
    BREACH = 0,
    COUNTER = 1,
    BOND = 2,
    TENSION = 3,
    ROUGH = 4,
    JUSTICE = 5,
    UNKNOWN = 6,
}

impl From<i32> for Style {
    fn from(value: i32) -> Self {
        match value {
            0 => Style::BREACH,
            1 => Style::COUNTER,
            2 => Style::BOND,
            3 => Style::TENSION,
            4 => Style::ROUGH,
            5 => Style::JUSTICE,
            _ => Style::UNKNOWN,
        }
    }
}

#[derive(Debug, Serialize, Clone, Copy, Default)]
pub struct Stats {
    pub kick: u16,
    pub control: u16,
    pub technique: u16,
    pub pressure: u16,
    pub physical: u16,
    pub agility: u16,
    pub intelligence: u16,
}

impl TryFrom<SqliteRow> for Stats {
    type Error = sqlx::Error;

    fn try_from(value: SqliteRow) -> Result<Self, Self::Error> {
        Ok(Stats {
            kick: value.try_get("lvl50_kick")?,
            control: value.try_get("lvl50_control")?,
            technique: value.try_get("lvl50_technique")?,
            pressure: value.try_get("lvl50_pressure")?,
            physical: value.try_get("lvl50_physical")?,
            agility: value.try_get("lvl50_agility")?,
            intelligence: value.try_get("lvl50_intelligence")?,
        })
    }
}
