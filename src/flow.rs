use crate::cause::*;

pub enum Flow {
    Continue,
    Pause(Cause),
    Exit,
}
