/*
 * SPDX-FileCopyrightText: 2023 Inria
 * SPDX-FileCopyrightText: 2023 Sebastiano Vigna
 * SPDX-FileCopyrightText: 2023 Tommaso Fontana
 *
 * SPDX-License-Identifier: Apache-2.0 OR LGPL-2.1-or-later
 */

use lender::*;

/// Iteration on nodes and associated labels.
///
/// This trait is a [`Lender`] returning pairs given by a `usize` (a node of the
/// graph) and an [`IntoIterator`], specified by the associated type `IntoIterator`,
/// over the labels associated with that node,
/// specified by the associated type `Label` (which is forced to be identical
/// to the associated type `Item` of the [`IntoIterator`]).
///
/// For those types we provide convenience type aliases [`LenderIntoIterator`],
/// [`LenderIntoIter`], and [`LenderLabel`].
///
/// ## Propagation of implicit bounds
///
/// The definition of this trait emerged from a [discussion on the Rust language
/// forum](https://users.rust-lang.org/t/more-help-for-more-complex-lifetime-situation/103821/10).
/// The purpose of the trait is to propagate the implicit
/// bound appearing in the definition [`Lender`] to the iterator returned
/// by the associated type [`IntoIterator`]. In this way, one can return iterators
/// depending on the internal state of the labeling. Without this additional trait, it
/// would be possible to return iterators whose state depends on the state of
/// the lender, but not on the state of the labeling.
pub trait NodeLabelsLender<'lend, __ImplBound: lender::ImplBound = lender::Ref<'lend, Self>>:
    Lender + Lending<'lend, __ImplBound, Lend = (usize, Self::IntoIterator)>
{
    type Label;
    type IntoIterator: IntoIterator<Item = Self::Label>;
}

/// Convenience type alias for the associated type `Label` of a [`NodeLabelsLender`].
pub type LenderLabel<'lend, L> = <L as NodeLabelsLender<'lend>>::Label;

/// Convenience type alias for the associated type `IntoIterator` of a [`NodeLabelsLender`].
pub type LenderIntoIterator<'lend, L> = <L as NodeLabelsLender<'lend>>::IntoIterator;

/// Convenience type alias for the [`Iterator`] returned by the `IntoIterator`
/// associated type of a [`NodeLabelsLender`].
pub type LenderIntoIter<'lend, L> =
    <<L as NodeLabelsLender<'lend>>::IntoIterator as IntoIterator>::IntoIter;

impl<'lend, A, B> NodeLabelsLender<'lend> for lender::Chain<A, B>
where
    A: Lender + for<'next> NodeLabelsLender<'next>,
    B: Lender
        + for<'next> NodeLabelsLender<
            'next,
            Label = <A as NodeLabelsLender<'next>>::Label,
            IntoIterator = <A as NodeLabelsLender<'next>>::IntoIterator,
        >,
{
    type Label = <A as NodeLabelsLender<'lend>>::Label;
    type IntoIterator = <A as NodeLabelsLender<'lend>>::IntoIterator;
}

impl<'lend, L, F, L2> NodeLabelsLender<'lend> for lender::Map<L, F>
where
    F: for<'all> lender::higher_order::FnMutHKA<
        'all,
        (usize, <L as NodeLabelsLender<'all>>::IntoIterator),
        B = (usize, L2),
    >,
    L2: IntoIterator,
    L: Lender + for<'next> NodeLabelsLender<'next>,
{
    type Label = L2::Item;
    type IntoIterator = L2;
}

impl<'lend, L> NodeLabelsLender<'lend> for lender::Take<L>
where
    L: Lender + for<'next> NodeLabelsLender<'next>,
{
    type Label = LenderLabel<'lend, L>;
    type IntoIterator = <L as NodeLabelsLender<'lend>>::IntoIterator;
}

impl<'lend, L, P> NodeLabelsLender<'lend> for lender::Filter<L, P>
where
    P: for<'next> FnMut(&(usize, <L as NodeLabelsLender<'next>>::IntoIterator)) -> bool,
    L: Lender + for<'next> NodeLabelsLender<'next>,
{
    type Label = LenderLabel<'lend, L>;
    type IntoIterator = <L as NodeLabelsLender<'lend>>::IntoIterator;
}
