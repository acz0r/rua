use ra_ap_hir::Semantics;
use ra_ap_ide_db::RootDatabase;
use ra_ap_vfs::{FileId, Vfs, VfsPath};
use rua::{RuaArgs, RuaResult};

pub struct RuaWorkspace<'w> {
    sema: Semantics<'w, RootDatabase>,
    vfs: Vfs,
    source_files: Vec<SourceFile<'w>>,
}

impl<'a> RuaWorkspace<'a> {
    pub fn load(args: RuaArgs) -> RuaResult<Self> {
        todo!()
    }
}

pub struct SourceFile<'s> {
    vfs: &'s Vfs,
    file_id: FileId,
    vfs_path: &'s VfsPath,
}
