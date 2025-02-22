use std::collections::HashMap;

use self::store::TyId;

use super::ctx::Ctx;
use super::ident::IdentId;
use super::mem::Layout;
use super::strukt::StructId;
use super::FuncHash;

pub mod infer;
pub mod store;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Ty {
    Int(IntTy),
    Struct(StructId),
    Unit,
}

impl Ty {
    pub fn is_int(&self) -> bool {
        matches!(self, Self::Int(_))
    }

    #[track_caller]
    pub fn expect_int(&self) -> IntTy {
        match self {
            Self::Int(ty) => *ty,
            _ => panic!("expected int"),
        }
    }

    #[track_caller]
    pub fn expect_struct(&self) -> StructId {
        match self {
            Self::Struct(s) => *s,
            _ => panic!("expected struct"),
        }
    }

    pub fn as_str<'a>(&self, ctx: &'a Ctx<'a>) -> &'a str {
        match self {
            Self::Unit => "()",
            Self::Int(int) => int.as_str(),
            Self::Struct(s) => ctx.expect_ident(ctx.tys.strukt(*s).name.id),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IntTy {
    pub sign: Sign,
    pub width: IWidth,
}

impl IntTy {
    pub fn new(sign: Sign, width: IWidth) -> Self {
        Self { sign, width }
    }

    pub fn new_8(sign: Sign) -> Self {
        Self::new(sign, IWidth::W8)
    }

    pub fn new_16(sign: Sign) -> Self {
        Self::new(sign, IWidth::W16)
    }

    pub fn new_32(sign: Sign) -> Self {
        Self::new(sign, IWidth::W32)
    }

    pub fn new_64(sign: Sign) -> Self {
        Self::new(sign, IWidth::W64)
    }

    pub fn size(&self) -> usize {
        self.width.bytes()
    }

    pub fn layout(&self) -> Layout {
        Layout::splat(self.size())
    }

    pub fn as_str(&self) -> &'static str {
        match self.sign {
            Sign::I => match self.width {
                IWidth::W8 => "i8",
                IWidth::W16 => "i16",
                IWidth::W32 => "i32",
                IWidth::W64 => "i64",
            },
            Sign::U => match self.width {
                IWidth::W8 => "u8",
                IWidth::W16 => "u16",
                IWidth::W32 => "u32",
                IWidth::W64 => "u64",
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Sign {
    I,
    U,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IWidth {
    W8,
    W16,
    W32,
    W64,
}

impl IWidth {
    pub fn bytes(&self) -> usize {
        match self {
            Self::W8 => 1,
            Self::W16 => 2,
            Self::W32 => 4,
            Self::W64 => 8,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TyVar(usize);

#[derive(Debug, Default)]
pub struct TypeKey {
    key: HashMap<VarHash, TyId>,
}

// TODO: rename to VarPath
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VarHash {
    ident: IdentId,
    func: FuncHash,
}

impl TypeKey {
    pub fn insert(&mut self, var: VarHash, ty: TyId) {
        self.key.insert(var, ty);
    }

    #[track_caller]
    pub fn ty(&self, ident: IdentId, func: FuncHash) -> TyId {
        *self
            .key
            .get(&VarHash { ident, func })
            .expect("variable is not keyed")
    }
}

//#[derive(Debug, Default)]
//pub struct TyCtx {
//    consts: Vec<Vec<Constraint>>,
//    vars: HashMap<VarHash, TyVar>,
//}
//
//impl TyCtx {
//    pub fn var(&mut self, ident: IdentId, func: FuncHash) -> TyVar {
//        let idx = self.consts.len();
//        self.vars.insert(VarHash { ident, func }, TyVar(idx));
//        self.consts.push(Vec::new());
//        TyVar(idx)
//    }
//
//    pub fn try_get_var(&self, ident: IdentId, func: FuncHash) -> Option<TyVar> {
//        self.vars.get(&VarHash { ident, func }).copied()
//    }
//
//    #[track_caller]
//    pub fn get_var(&self, ident: IdentId, func: FuncHash) -> TyVar {
//        self.try_get_var(ident, func).expect("invalid ident")
//    }
//
//    #[track_caller]
//    pub fn constrain(&mut self, ty_var: TyVar, constraint: Constraint) {
//        self.consts
//            .get_mut(ty_var.0)
//            .expect("invalid ty var")
//            .push(constraint);
//    }
//
//    pub fn resolve(&self, ctx: &Ctx) -> Result<TypeKey, Vec<TyErr>> {
//        let map = self
//            .vars
//            .iter()
//            .map(|(ident, var)| (*var, *ident))
//            .collect::<HashMap<_, _>>();
//
//        let mut key = HashMap::with_capacity(self.consts.len());
//        let mut errs = Vec::new();
//        for (i, c) in self.consts.iter().enumerate() {
//            let var = TyVar(i);
//            match Constraint::unify(self, ctx, var, c) {
//                Ok(ty) => {
//                    key.insert(*map.get(&var).unwrap(), ty);
//                }
//                Err(err) => errs.push(err),
//            }
//        }
//
//        if !errs.is_empty() {
//            Err(errs)
//        } else {
//            Ok(TypeKey { key })
//        }
//    }
//}

//#[derive(Debug)]
//pub struct TyRegistry<'a> {
//    symbol_map: HashMap<&'a str, TyId>,
//    ty_map: HashMap<TyId, Ty>,
//    tys: Vec<&'a str>,
//}
//
//impl Default for TyRegistry<'_> {
//    fn default() -> Self {
//        let mut slf = Self {
//            symbol_map: HashMap::default(),
//            ty_map: HashMap::default(),
//            tys: Vec::new(),
//        };
//
//        slf.register_ty("i8", Ty::Int(IntKind::I8));
//        slf.register_ty("i16", Ty::Int(IntKind::I16));
//        slf.register_ty("i32", Ty::Int(IntKind::I32));
//        slf.register_ty("i64", Ty::Int(IntKind::I64));
//
//        slf.register_ty("u8", Ty::Int(IntKind::U8));
//        slf.register_ty("u16", Ty::Int(IntKind::U16));
//        slf.register_ty("u32", Ty::Int(IntKind::U32));
//        slf.register_ty("u64", Ty::Int(IntKind::U64));
//
//        slf
//    }
//}
//
//impl<'a> TyRegistry<'a> {
//    pub fn ty_str(&self, ty: &'a str) -> Option<Ty> {
//        self.symbol_map
//            .get(ty)
//            .map(|id| self.ty_map.get(id).copied())?
//    }
//
//    fn register_ty(&mut self, ty_str: &'a str, ty: Ty) {
//        let id = TyId(self.tys.len());
//        self.ty_map.insert(id, ty);
//        self.tys.push(ty_str);
//        self.symbol_map.insert(ty_str, id);
//    }
//}
//
//#[derive(Debug, Clone)]
//pub struct Constraint {
//    pub span: Span,
//    pub kind: ConstraintKind,
//}
//
//#[derive(Debug, Clone)]
//pub enum ConstraintKind {
//    Arch(Arch),
//    Equate(TyVar),
//    Abs(Ty),
//    Struct(StructId),
//    EnumVariant(IdentId, IdentId),
//    // TODO: this makes me want to throw up
//    Field(Vec<IdentId>, Box<Constraint>),
//}
//
//impl ConstraintKind {
//    pub fn full(ty: Ty) -> Self {
//        match ty {
//            Ty::Ty(ty) => Self::Abs(ty),
//            Ty::Struct(s) => Self::Struct(s),
//        }
//    }
//
//    pub fn hint_satisfies(&self, ty: Ty) -> Option<bool> {
//        match self {
//            Self::Arch(arch) => Some(ty.is_ty_and(|ty| arch.satisfies(*ty))),
//            Self::Abs(abs) => Some(ty.is_ty_and(|ty| abs == ty)),
//            Self::Struct(strukt) => Some(ty.is_struct_and(|s| s == strukt)),
//            Self::EnumVariant(_, _) => todo!(),
//            Self::Field(_, _) => None,
//            Self::Equate(_) => None,
//        }
//    }
//
//    pub fn is_int(&self) -> Option<bool> {
//        match self {
//            Self::Arch(arch) => Some(matches!(arch, Arch::Int)),
//            Self::Abs(abs) => Some(abs.is_int()),
//            Self::Struct(_) => Some(false),
//            Self::EnumVariant(_, _) => Some(false),
//            Self::Field(_, _) => None,
//            Self::Equate(_) => None,
//        }
//    }
//}
//
//#[derive(Debug)]
//pub enum TyErr {
//    NotEnoughInfo(Span, TyVar),
//    Arch(Span, Arch, Ty),
//    Abs(Span),
//    Struct(StructId),
//}
//
//impl Constraint {
//    pub fn unify(
//        ty_ctx: &TyCtx,
//        ctx: &Ctx,
//        var: TyVar,
//        constraints: &[Constraint],
//    ) -> Result<Ty, TyErr> {
//        //println!("{:#?}", ty_ctx);
//        let mut archs = Vec::with_capacity(constraints.len());
//        let mut abs = None;
//        let mut enom = None;
//        let mut strukt = None;
//        let mut field_constraints = HashMap::<&[IdentId], Vec<Box<Constraint>>>::new();
//
//        let mut constraints = constraints.to_vec();
//        Self::resolve_equates(ty_ctx, ctx, &mut constraints, &mut vec![var]);
//
//        for c in constraints.iter() {
//            match &c.kind {
//                ConstraintKind::Abs(ty) => {
//                    if abs.is_some_and(|abs| abs != *ty) {
//                        return Err(TyErr::Abs(c.span));
//                    }
//
//                    abs = Some(*ty);
//                }
//                ConstraintKind::Arch(a) => archs.push((c.span, a)),
//                ConstraintKind::Struct(s) => {
//                    if strukt.is_some_and(|ident| ident != *s) {
//                        return Err(TyErr::Struct(*s));
//                    }
//
//                    strukt = Some(*s);
//                }
//                ConstraintKind::EnumVariant(eno, variant) => {
//                    if enom.is_some_and(|(other_e, other_v)| *eno != other_e || *variant != other_v)
//                    {
//                        todo!();
//                        //return Err(TyErr::Struct(*s));
//                    }
//
//                    enom = Some((*eno, *variant));
//                }
//                ConstraintKind::Field(path, constraint) => {
//                    field_constraints
//                        .entry(&path)
//                        .or_default()
//                        .push(constraint.clone());
//                }
//                _ => unreachable!(),
//            }
//        }
//
//        if let Some(abs) = abs {
//            if strukt.is_some() {
//                return Err(TyErr::Struct(strukt.unwrap()));
//            }
//
//            if enom.is_some() {
//                todo!()
//            }
//
//            for (span, arch) in archs.iter() {
//                if !arch.satisfies(abs) {
//                    return Err(TyErr::Arch(*span, **arch, abs));
//                }
//            }
//
//            if !field_constraints.is_empty() {
//                panic!("accessor on a primitive");
//            }
//
//            Ok(Ty::Ty(abs))
//        } else if let Some((enom, _)) = enom {
//            todo!()
//            //if strukt.is_some() {
//            //    return Err(TyErr::Struct(strukt.unwrap()));
//            //}
//            //
//            //Ok(Ty::Struct(enom))
//        } else if let Some(strukt) = strukt {
//            for (_span, _arch) in archs.iter() {
//                panic!("type err");
//                //return Err(TyErr::Arch(*span, *arch, abs));
//            }
//
//            let mut struct_def = ctx.structs.strukt(strukt);
//            for (path, constraints) in field_constraints.iter() {
//                for (i, field) in path.iter().enumerate() {
//                    if let Some(ty) = struct_def.get_field_ty(*field) {
//                        match ty {
//                            Ty::Ty(ty) => {
//                                if path.len() - 1 == i {
//                                    let mut constraints = constraints
//                                        .iter()
//                                        .map(|c| c.as_ref().clone())
//                                        .collect::<Vec<_>>();
//                                    constraints.push(Constraint {
//                                        span: struct_def.span,
//                                        kind: ConstraintKind::Abs(ty),
//                                    });
//
//                                    Constraint::unify(
//                                        ty_ctx,
//                                        ctx,
//                                        TyVar(usize::MAX),
//                                        &constraints,
//                                    )?;
//                                } else {
//                                    todo!("invalid field path: {:#?}", path);
//                                }
//                            }
//                            Ty::Struct(s) => {
//                                struct_def = ctx.structs.strukt(s);
//                            }
//                        }
//                    } else {
//                        todo!("invalid field path: {:#?}", path);
//                    }
//                }
//            }
//
//            Ok(Ty::Struct(strukt))
//        } else {
//            panic!("not enough info");
//            //ctx.exprs
//            //    .iter()
//            //    .find(|expr| expr.ty == var)
//            //    .map(|expr| TyErr::NotEnoughInfo(expr.span, var))
//            //    .unwrap_or_else(|| panic!("no one uses this ty var?"))
//        }
//    }
//
//    fn resolve_equates(
//        ty_ctx: &TyCtx,
//        ctx: &Ctx,
//        constraints: &mut Vec<Constraint>,
//        resolved: &mut Vec<TyVar>,
//    ) {
//        let mut new_constraints = Vec::<Constraint>::new();
//
//        *constraints = constraints
//            .iter()
//            .cloned()
//            .flat_map(|c| match c.kind {
//                ConstraintKind::Equate(other) => {
//                    if !resolved.contains(&other) {
//                        resolved.push(other);
//                        new_constraints.extend(
//                            ty_ctx.consts[other.0]
//                                .iter()
//                                .cloned()
//                                .filter(|c| {
//                                    if let ConstraintKind::Equate(other) = c.kind {
//                                        !resolved.contains(&other)
//                                    } else {
//                                        true
//                                    }
//                                })
//                                .clone(),
//                        )
//                    }
//
//                    None
//                }
//                _ => Some(c),
//            })
//            .collect::<Vec<_>>();
//
//        if !new_constraints.is_empty() {
//            constraints.extend(new_constraints);
//            Self::resolve_equates(ty_ctx, ctx, constraints, resolved);
//        }
//    }
//}
//
//#[derive(Debug, Clone, Copy)]
//pub enum Arch {
//    Int,
//}
//
//impl Arch {
//    pub fn as_str(&self) -> &'static str {
//        match self {
//            Self::Int => "int",
//        }
//    }
//
//    pub fn satisfies(&self, ty: Ty) -> bool {
//        match self {
//            Arch::Int => ty.is_int(),
//        }
//    }
//}
