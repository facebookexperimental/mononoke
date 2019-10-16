/*
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This software may be used and distributed according to the terms of the
 * GNU General Public License found in the LICENSE file in the root
 * directory of this source tree.
 */

use failure::Fail;
use std::collections::HashSet;

pub use mercurial_types::HgChangesetId;
use metaconfig_types::BookmarkOrRegex;
pub use mononoke_types::MPath;

#[derive(Debug, Fail)]
pub enum ErrorKind {
    #[fail(display = "No such hook '{}'", _0)]
    NoSuchHook(String),

    #[fail(display = "Error while parsing hook '{}'", _0)]
    HookParseError(String),
    #[fail(display = "Error while running hook '{}'", _0)]
    HookRuntimeError(String),

    #[fail(display = "invalid file structure: {}", _0)]
    InvalidFileStructure(String),
    #[fail(display = "invalid path: {}", _0)]
    InvalidPath(MPath),

    #[fail(display = "Missing file for cs '{}' path '{}'", _0, _1)]
    MissingFile(HgChangesetId, MPath),

    #[fail(
        display = "Hook(s) referenced in bookmark {:#?} do not exist: {:?}",
        _0, _1
    )]
    NoSuchBookmarkHook(BookmarkOrRegex, HashSet<String>),

    #[fail(display = "invalid rust hook: {}", _0)]
    InvalidRustHook(String),

    #[fail(display = "Disabled hook(s) do(es) not exist: {:?}", _0)]
    NoSuchHookToDisable(HashSet<String>),
}
