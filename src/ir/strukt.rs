use super::ctx::Ctx;
use super::ident::{Ident, IdentId};
use super::ty::store::TyId;
use super::Expr;
use crate::lex::buffer::Span;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Struct {
    pub span: Span,
    pub name: Ident,
    pub fields: Vec<Field>,
}

impl Struct {
    pub fn get_field_ty(&self, field: IdentId) -> Option<TyId> {
        self.fields
            .iter()
            .find(|f| f.name.id == field)
            .map(|f| f.ty)
    }

    #[track_caller]
    pub fn field_ty(&self, field: IdentId) -> TyId {
        self.get_field_ty(field).expect("invalid field")
    }

    #[track_caller]
    pub fn field_offset(&self, ctx: &Ctx, field: IdentId) -> i32 {
        let map = ctx.tys.fields(ctx.expect_struct_id(self.name.id));
        map.fields.get(&field).expect("invalid field").1
    }
}

#[derive(Debug, Clone)]
pub struct Field {
    pub span: Span,
    pub name: Ident,
    pub ty: TyId,
}

#[derive(Debug, Clone)]
pub struct StructDef {
    pub span: Span,
    pub id: StructId,
    pub fields: Vec<FieldDef>,
}

#[derive(Debug, Clone)]
pub struct FieldDef {
    pub span: Span,
    pub name: Ident,
    pub expr: Expr,
}

#[derive(Debug, Clone)]
pub struct FieldMap {
    pub fields: HashMap<IdentId, (TyId, ByteOffset)>,
}

pub type ByteOffset = i32;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StructId(pub(super) usize);
