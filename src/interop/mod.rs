mod error;
pub use self::error::Error;

use std::io::Read;
use types::{Message, Variation, Number};
use types::system::*;
use types::transform::*;
use types::affine_transformation::*;
use types::coloring_method;
use variations;

use chaoskit_capnp::{message, MessageType, flame, transform, variation, affine_transformation};
use capnp;
use capnp::serialize;

pub fn read_message<R: Read>(reader: &mut R) -> Result<Message, Error> {
    let message_reader = try!(serialize::read_message(reader, capnp::message::ReaderOptions::new()));
    let msg: message::Reader = try!(message_reader.get_root());

    match msg.get_type() {
        Ok(MessageType::Start) => match msg.get_body().which() {
            Ok(message::body::Flame(flame_result)) => {
                let flame_reader = try!(flame_result);
                let flame = try!(read_flame(flame_reader));
                Ok(Message::Start(flame))
            }
            _ => Err(Error::new("Invalid message body"))
        },
        Ok(MessageType::Stop) => Ok(Message::Stop),
        Err(_) => Err(Error::new("Invalid message type"))
    }
}

fn read_flame(flame: flame::Reader) -> Result<System, Error> {
    let mut builder = SystemBuilder::new();

    for transform_reader in try!(flame.get_transforms()).iter() {
        let weight = transform_reader.get_weight();
        let transform = try!(read_transform(transform_reader));

        builder = builder.add_weighted_transform(transform, weight);
    }

    if flame.has_final_transform() {
        let final_transform = try!(flame.get_final_transform());
        let transform = try!(read_transform(final_transform));

        builder = builder.final_transform(transform);
    }

    if flame.has_reset_transformation() {
        let af_reader = try!(flame.get_reset_transformation());
        let af = try!(read_affine_transformation(af_reader));

        builder = builder.reset_transformation(af);
    }

    let ttl = flame.get_ttl();

    Ok(builder.ttl(ttl).finalize())
}

fn read_transform(transform: transform::Reader) -> Result<Transform, Error> {
    let mut builder = TransformBuilder::new();

    for variation_reader in try!(transform.get_variations()).iter() {
        let variation: Box<Variation> = try!(read_variation(variation_reader));

        if variation_reader.has_weight() {
            let weight = try!(variation_reader.get_weight());
            builder = builder.add_boxed_2d_weighted_variation(variation, weight.get_x() as Number, weight.get_y() as Number);
        } else {
            builder = builder.add_boxed_variation(variation);
        }
    }

    if transform.has_pre() {
        builder = builder.pre(try!(read_affine_transformation(try!(transform.get_pre()))));
    }

    if transform.has_post() {
        builder = builder.post(try!(read_affine_transformation(try!(transform.get_post()))));
    }

    match transform.get_coloring_method().which() {
        Ok(transform::coloring_method::Noop(())) => {
            builder = builder.coloring_method(Box::new(coloring_method::Noop));
        },
        Ok(transform::coloring_method::Distance(())) => {
            builder = builder.coloring_method(Box::new(coloring_method::Distance));
        },
        Ok(transform::coloring_method::SingleColor(color)) => {
            builder = builder.coloring_method(Box::new(coloring_method::SingleColor::new(color as Number)));
        },
        Err(capnp::NotInSchema(_)) => {
            return Err(Error::new("Invalid coloring method"));
        }
    }

    Ok(builder.finalize())
}

fn read_variation(variation: variation::Reader) -> Result<Box<Variation>, Error> {
    let name = try!(variation.get_name());
    let params = try!(variation.get_params());

    // Copy params into a Vec
    let mut params_vec = Vec::with_capacity(params.len() as usize);
    for i in 0..params.len() {
        params_vec.push(params.get(i) as Number);
    }

    variations::make_variation(name, &params_vec).map_err(|e| Error::from(e))
}

fn read_affine_transformation(at: affine_transformation::Reader) -> Result<AffineTransformation, Error> {
    let translation = try!(at.get_translation());
    let scale = try!(at.get_scale());

    Ok(AffineTransformationBuilder::new()
        .rotation(at.get_rotation() as Number)
        .translation(translation.get_x() as Number, translation.get_y() as Number)
        .scale(scale.get_x() as Number, scale.get_y() as Number)
        .finalize())
}
