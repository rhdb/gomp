use std::fmt::Debug;

#[derive(Debug, Default)]
pub struct Yes;
#[derive(Debug, Default)]
pub struct No;

pub trait ToAssign: Debug {}
pub trait Assigned: ToAssign {}
pub trait NotAssigned: ToAssign {}

impl ToAssign for Yes {}
impl ToAssign for No {}

impl Assigned for Yes {}
impl NotAssigned for No {}

