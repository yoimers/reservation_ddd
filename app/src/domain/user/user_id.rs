use sqlx::{Postgres, Type};

use super::user::User;
use crate::domain::common::id::ID;

pub type UserID = ID<User>;
