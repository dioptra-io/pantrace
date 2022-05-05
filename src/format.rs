use crate::TracerouteReply;

pub trait PantraceFormat {
    // TODO: Return Result with error detail.
    fn from_bytes(data: &[u8]) -> Option<Self>
    where
        Self: Sized;
    fn to_bytes(self) -> Vec<u8>;
    // TODO: Default impl. where we assert flow consistency?
    //   => checked/unchecked versions?
    fn from_internal_or_none(replies: &[TracerouteReply]) -> Option<Self>
    where
        Self: Sized,
    {
        if replies.is_empty() {
            None
        } else {
            Some(Self::from_internal(replies))
        }
    }
    fn from_internal(replies: &[TracerouteReply]) -> Self
    where
        Self: Sized;
    fn to_internal(&self) -> Vec<TracerouteReply>;
}
