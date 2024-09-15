use std::error::Error;
use std::fmt;
use std::fmt::Debug;

#[derive(Debug)]
pub struct FileSizeLimitError;

impl std::fmt::Display for FileSizeLimitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "file exceeds host's filesize limit")
    }
}

impl Error for FileSizeLimitError {}

#[derive(Debug)]
pub struct BadAPIResponseError;

impl std::fmt::Display for crate::hosts::errors::BadAPIResponseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "the api reported failure")
    }
}

impl Error for BadAPIResponseError {}


pub struct SectionKeyMissingError {
    message: String,
}

impl SectionKeyMissingError {
    pub fn new(message: &str) -> Self {
        SectionKeyMissingError {
            message: message.to_string(),
        }
    }
}

impl fmt::Debug for SectionKeyMissingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "required section key \"{}\" is missing from the config file", self.message)
    }
}

impl fmt::Display for SectionKeyMissingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for SectionKeyMissingError {}

pub struct SectionValueEmptyError {
    message: String,
}

impl SectionValueEmptyError {
    pub fn new(message: &str) -> Self {
        SectionValueEmptyError {
            message: message.to_string(),
        }
    }
}

// Implement the Display trait for the struct
impl fmt::Display for SectionValueEmptyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"{}\"", self.message)
    }
}

impl fmt::Debug for SectionValueEmptyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "required value for key \"{}\" is empty in config file", self.message)
    }
}
impl Error for SectionValueEmptyError {}