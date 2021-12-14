// Generated by diesel_ext

#![allow(unused)]
#![allow(clippy::all)]

use serde::Serialize;
use chrono::NaiveDate;
use crate::schema::*;
use crate::User_role;

#[derive(Queryable, Debug, Identifiable)]
pub struct Document {
    pub id: i32,
    pub patient_id: i32,
    pub institution_id: Option<i32>,
    pub date_a: Option<NaiveDate>,
    pub date_b: Option<NaiveDate>,
    pub date_c: Option<NaiveDate>,
    pub date_d: Option<NaiveDate>,
    pub diagnosis: Option<String>,
    pub anamnesis: Option<String>,
    pub recommendations: Option<String>,
    pub document_date: Option<NaiveDate>,
}

#[derive(Queryable, Debug, Identifiable)]
pub struct MedicalInstitution {
    pub id: i32,
    pub fullname: String,
    pub address: Option<String>,
}

#[derive(Queryable, Debug, Identifiable)]
pub struct Patient {
    pub id: i32,
    pub first_name: String,
    pub second_name: Option<String>,
    pub last_name: String,
    pub dob: NaiveDate,
    pub address: String,
    pub diagnosis: Option<String>,
    pub diagnosis_code: Option<String>,
    pub occupation: Option<String>,
}

#[derive(Queryable, Debug, Identifiable, Serialize)]
#[primary_key(key)]
pub struct User {
    pub key: i32,
    pub login: String,
    pub role: User_role,
    pub salt: String,
    pub hash: String,
}