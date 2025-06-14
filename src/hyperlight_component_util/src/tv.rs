/*
Copyright 2025 The Hyperlight Authors.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
 */

use crate::etypes::{
    BoundedTyvar, Ctx, Defined, FreeTyvar, Handleable, ImportExport, TypeBound, Tyvar,
};
use crate::substitute::{self, Substitution, Unvoidable};

/// The most information we possibly have about a type variable
pub enum ResolvedTyvar<'a> {
    /// Invariant: the head of this [`Defined`] is not `[Defined::Handleable]([HHandleable::Var](...))`
    Definite(Defined<'a>),
    /// It's just some bound var... so there is no way to look it up.
    #[allow(unused)]
    Bound(u32),
    /// Invariant: the `TypeBound` is not `TypeBound::Eq`
    E(u32, u32, TypeBound<'a>),
    /// Invariant: the `TypeBound` is not `TypeBound::Eq`
    U(u32, u32, TypeBound<'a>),
}

impl<'p, 'a> Ctx<'p, 'a> {
    /// Look up a universal variable in the context, panicking if it doesn't exist
    fn lookup_uvar<'c>(&'c self, o: u32, i: u32) -> &'c (BoundedTyvar<'a>, bool) {
        // unwrap because failure is an internal invariant violation
        &self.parents().nth(o as usize).unwrap().uvars[i as usize]
    }
    /// Look up an existential variable in the context, panicking if it doesn't exist
    fn lookup_evar<'c>(&'c self, o: u32, i: u32) -> &'c (BoundedTyvar<'a>, Option<Defined<'a>>) {
        // unwrap because failure is an internal invariant violation
        &self.parents().nth(o as usize).unwrap().evars[i as usize]
    }
    /// Find a bound for the given free tyvar. Panics if given a
    /// TV_bound; by the time you call this, you should have used
    /// bound_to_[e/u]var.
    pub fn var_bound<'c>(&'c self, tv: &Tyvar) -> &'c TypeBound<'a> {
        match tv {
            Tyvar::Bound(_) => panic!("Requested bound for Bound tyvar"),
            Tyvar::Free(FreeTyvar::U(o, i)) => &self.lookup_uvar(*o, *i).0.bound,
            Tyvar::Free(FreeTyvar::E(o, i)) => &self.lookup_evar(*o, *i).0.bound,
        }
    }
    /// Try really hard to resolve a tyvar to a definite type or a
    /// descriptive bound.
    pub fn resolve_tyvar<'c>(&'c self, v: &Tyvar) -> ResolvedTyvar<'a> {
        let check_deftype = |dt: &Defined<'a>| match dt {
            Defined::Handleable(Handleable::Var(v_)) => self.resolve_tyvar(v_),
            _ => ResolvedTyvar::Definite(dt.clone()),
        };
        match *v {
            Tyvar::Bound(i) => ResolvedTyvar::Bound(i),
            Tyvar::Free(FreeTyvar::E(o, i)) => {
                let (tv, def) = self.lookup_evar(o, i);
                match (&tv.bound, def) {
                    (TypeBound::Eq(dt), _) => check_deftype(dt),
                    (_, Some(dt)) => check_deftype(dt),
                    (tb, _) => ResolvedTyvar::E(o, i, tb.clone()),
                }
            }
            Tyvar::Free(FreeTyvar::U(o, i)) => {
                let (tv, _) = self.lookup_uvar(o, i);
                match &tv.bound {
                    TypeBound::Eq(dt) => check_deftype(dt),
                    tb => ResolvedTyvar::U(o, i, tb.clone()),
                }
            }
        }
    }
    /// Modify the context to move the given variables into it as
    /// existential variables and compute a substitution
    /// that replaces bound variable references to them with free
    /// variable references
    pub fn bound_to_evars(
        &mut self,
        origin: Option<&'a str>,
        vs: &[BoundedTyvar<'a>],
    ) -> substitute::Opening {
        let mut sub = substitute::Opening::new(false, self.evars.len() as u32);
        for var in vs {
            let var = var.push_origin(origin.map(ImportExport::Export));
            let bound = sub.bounded_tyvar(&var).not_void();
            self.evars.push((bound, None));
            sub.next();
        }
        sub
    }
    /// Modify the context to move the given variables into it as
    /// universal variables and compute a substitution that replaces
    /// bound variable references to them with free variable
    /// references
    pub fn bound_to_uvars(
        &mut self,
        origin: Option<&'a str>,
        vs: &[BoundedTyvar<'a>],
        imported: bool,
    ) -> substitute::Opening {
        let mut sub = substitute::Opening::new(true, self.uvars.len() as u32);
        for var in vs {
            let var = var.push_origin(origin.map(ImportExport::Import));
            let bound = sub.bounded_tyvar(&var).not_void();
            self.uvars.push((bound, imported));
            sub.next();
        }
        sub
    }
}
