//
// imag - the personal information management suite for the commandline
// Copyright (C) 2015, 2016 Matthias Beyer <mail@beyermatthias.de> and contributors
//
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public
// License as published by the Free Software Foundation; version
// 2.1 of the License.
//
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public
// License along with this library; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301  USA
//

use libimagstore::store::Store;
use libimagstore::store::FileLockEntry;
use libimagstore::storeid::StoreId;

use result::Result;
use error::AnnotationErrorKind as AEK;

use module_path::ModuleEntryPath;

pub trait Annotateable {

    /// Add an annotation to `Self`, that is a `FileLockEntry` which is linked to `Self` (link as in
    /// libimagentrylink).
    ///
    /// This `Annotation` is stored in the Store itself.
    fn annotate(&self, store: &Store) -> Result<Annotation> {
        self.annotate_with_path_generator(store, DefaultAnnotationPathGenerator::new())
    }

    /// Add an annotation to `Self`, that is a `FileLockEntry` which is linked to `Self` (link as in
    /// libimagentrylink).
    ///
    /// This `Annotation` is stored in the Store itself.
    ///
    /// The `pg` is a AnnotationPathGenerator object which is used to generate a StoreId
    fn annotate_with_path_generator(&self, store: &Store, pg: &AnnotationPathGenerator) -> Result<Annotation>;

}

/// A AnnotationPathGenerator generates a unique path for the annotation to be generated.
pub trait AnnotationPathGenerator {
    fn generate_annotation_path(&self) -> Result<StoreId>;
}

/// The DefaultAnnotationPathGenerator generates unique StoreIds for Annotations, where the
/// collection the annotations are stored in is `/annotation/`.
pub struct DefaultAnnotationPathGenerator;

impl AnnotationPathGenerator for DefaultAnnotationPathGenerator {

    fn generate_annotation_path(&self) -> Result<StoreId> {
        let id = Uuid::new_v4();
        ModuleEntryPath::new(format!("{}", id)).map_err_into(AEK::StoreIdGenerationError)
    }

}

pub struct Annotation<'a>(FileLockEntry<'a>);

impl Annotateable for FileLockEntry {

    fn annotate_with_path_generator(&self, store: &Store, pg: &AnnotationPathGenerator)
        -> Result<Annotation>
    {
    }
}