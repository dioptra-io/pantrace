use crate::TracerouteReply;

pub trait PantraceFormat {
    fn from_bytes(data: &[u8]) -> Option<Self>
    where
        Self: Sized;
    fn to_bytes(self) -> Vec<u8>;
    // TODO: Default impl. where we assert flow consistency?
    //   => checked/unchecked versions?
    fn from_internal(replies: &[TracerouteReply]) -> Option<Self>
    where
        Self: Sized;
    fn to_internal(&self) -> Vec<TracerouteReply>;
}
