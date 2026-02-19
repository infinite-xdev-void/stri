//
//
//

/// this is just a marker
pub trait Format {}

//
//
//
//
//
//
//

pub struct Sql;

impl Format for Sql {}

//

pub struct Str;

impl Format for Str {}
