// pathfinder/content/src/effects.rs
//
// Copyright © 2020 The Pathfinder Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Special effects that can be applied to layers.

use pathfinder_color::ColorF;

/// This intentionally does not precisely match what Core Graphics does (a
/// Lanczos function), because we don't want any ringing artefacts.
pub static DEFRINGING_KERNEL_CORE_GRAPHICS: DefringingKernel =
    DefringingKernel([0.033165660, 0.102074051, 0.221434336, 0.286651906]);
pub static DEFRINGING_KERNEL_FREETYPE: DefringingKernel =
    DefringingKernel([0.0, 0.031372549, 0.301960784, 0.337254902]);

/// Should match macOS 10.13 High Sierra.
pub static STEM_DARKENING_FACTORS: [f32; 2] = [0.0121, 0.0121 * 1.25];

/// Should match macOS 10.13 High Sierra.
pub const MAX_STEM_DARKENING_AMOUNT: [f32; 2] = [0.3, 0.3];

/// This value is a subjective cutoff. Above this ppem value, no stem darkening is performed.
pub const MAX_STEM_DARKENING_PIXELS_PER_EM: f32 = 72.0;

/// Effects that can be applied to a layer.
#[derive(Clone, Copy, Debug)]
pub struct Effects {
    /// The shader that should be used when compositing this layer onto its destination.
    pub filter: Filter,
}

/// The shader that should be used when compositing this layer onto its destination.
#[derive(Clone, Copy, Debug)]
pub enum Filter {
    /// A Porter-Duff compositing operation.
    ///
    /// The compositing operations here are the ones that can't be blend modes because they can
    /// clear parts of the destination not overlapped by the source, plus the regular source-over.
    Composite(CompositeOp),

    /// Performs postprocessing operations useful for monochrome text.
    Text {
        /// The foreground color of the text.
        fg_color: ColorF,
        /// The background color of the text.
        bg_color: ColorF,
        /// The kernel used for defringing, if subpixel AA is enabled.
        defringing_kernel: Option<DefringingKernel>,
        /// Whether gamma correction is used when compositing.
        ///
        /// If this is enabled, stem darkening is advised.
        gamma_correction: bool,
    },

    /// A blur operation in one direction, either horizontal or vertical.
    ///
    /// To produce a full Gaussian blur, perform two successive blur operations, one in each
    /// direction.
    Blur {
        direction: BlurDirection,
        sigma: f32,
    },
}

#[derive(Clone, Copy, Debug)]
pub enum CompositeOp {
    /// The default.
    SrcOver,
    /// No regions are enabled.
    Clear,
    /// Only the source will be present.
    Copy,
    /// The source that overlaps the destination replaces the destination.
    SrcIn,
    /// Destination which overlaps the source replaces the source.
    DestIn,
    /// Source is placed where it falls outside of the destination.
    SrcOut,
    /// Destination which overlaps the source replaces the source. Source is placed elsewhere.
    DestAtop,
}

/// Blend modes that can be applied to individual paths.
///
/// All blend modes preserve parts of the destination that are not overlapped by the source path.
/// Other Porter-Duff compositing operations are `CompositeOp`s.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum BlendMode {
    // Supported by GPU blender
    Clear,
    SrcOver,
    DestOver,
    DestOut,
    SrcAtop,
    Xor,
    Lighter,
    Lighten,
    Darken,

    // Overlay
    Multiply,
    Screen,
    HardLight,
    Overlay,

    // Dodge/burn
    ColorDodge,
    ColorBurn,

    // Soft light
    SoftLight,

    // Difference
    Difference,

    // Exclusion
    Exclusion,

    // HSL
    Hue,
    Saturation,
    Color,
    Luminosity,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct DefringingKernel(pub [f32; 4]);

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum BlurDirection {
    X,
    Y,
}

impl Default for CompositeOp {
    #[inline]
    fn default() -> CompositeOp {
        CompositeOp::SrcOver
    }
}

impl Default for BlendMode {
    #[inline]
    fn default() -> BlendMode {
        BlendMode::SrcOver
    }
}

impl Effects {
    #[inline]
    pub fn new(filter: Filter) -> Effects {
        Effects { filter }
    }
}

impl BlendMode {
    /// Whether the backdrop is irrelevant when applying this blend mode (i.e. destination blend
    /// factor is zero when source alpha is one).
    #[inline]
    pub fn occludes_backdrop(self) -> bool {
        match self {
            BlendMode::SrcOver | BlendMode::Clear => true,
            BlendMode::DestOver |
            BlendMode::DestOut |
            BlendMode::SrcAtop |
            BlendMode::Xor |
            BlendMode::Lighter |
            BlendMode::Lighten |
            BlendMode::Darken |
            BlendMode::Multiply |
            BlendMode::Screen |
            BlendMode::HardLight |
            BlendMode::Overlay |
            BlendMode::ColorDodge |
            BlendMode::ColorBurn |
            BlendMode::SoftLight |
            BlendMode::Difference |
            BlendMode::Exclusion |
            BlendMode::Hue |
            BlendMode::Saturation |
            BlendMode::Color |
            BlendMode::Luminosity => false,
        }
    }
}
