#[cfg(feature = "impl-abi_stable")]
mod abi_stable;
mod array;
#[cfg(feature = "impl-chrono")]
mod chrono;
mod collections;
mod deref;
mod primitives;
mod simdjson;
mod string;
mod tpl;
use crate::*;
use value_trait::generator::BaseGenerator;

struct DummyGenerator<W: Write>(W);
impl<W: Write> BaseGenerator for DummyGenerator<W> {
    type T = W;
    #[inline]
    fn get_writer(&mut self) -> &mut <Self as BaseGenerator>::T {
        &mut self.0
    }
    #[inline]
    fn write_min(&mut self, _: &[u8], _: u8) -> io::Result<()> {
        unimplemented!()
    }
}

impl<T> Serialize for Option<T>
where
    T: Serialize,
{
    #[inline]
    fn json_write<W>(&self, writer: &mut W) -> io::Result<()>
    where
        W: Write,
    {
        if let Some(e) = self {
            e.json_write(writer)
        } else {
            writer.write_all(b"null")
        }
    }
}

impl<'input, T> Deserialize<'input> for Option<T>
where
    T: Deserialize<'input>,
{
    #[inline]
    fn from_tape(tape: &mut Tape<'input>) -> simd_json::Result<Self>
    where
        Self: std::marker::Sized + 'input,
    {
        if let Some(simd_json::Node::Static(simd_json::StaticNode::Null)) = tape.peek() {
            tape.next();
            Ok(None)
        } else {
            Ok(Some(T::from_tape(tape)?))
        }
    }
}
