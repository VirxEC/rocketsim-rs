#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
mod scg {
    pub trait ToCppString {
        fn into_cpp(self) -> cxx::UniquePtr<cxx::CxxString>;
    }
    impl ToCppString for &str {
        fn into_cpp(self) -> cxx::UniquePtr<cxx::CxxString> {
            make_string(self)
        }
    }
    impl ToCppString for String {
        fn into_cpp(self) -> cxx::UniquePtr<cxx::CxxString> {
            make_string(&self)
        }
    }
    impl ToCppString for &String {
        fn into_cpp(self) -> cxx::UniquePtr<cxx::CxxString> {
            make_string(self)
        }
    }
    impl ToCppString for cxx::UniquePtr<cxx::CxxString> {
        fn into_cpp(self) -> cxx::UniquePtr<cxx::CxxString> {
            self
        }
    }
    unsafe impl cxx::ExternType for bindgen::root::SuspensionCollisionGrid {
        type Id = cxx::type_id!("SuspensionCollisionGrid");
        type Kind = cxx::kind::Opaque;
    }
    unsafe impl cxx::ExternType for bindgen::root::SuspensionCollisionGrid_Cell {
        type Id = cxx::type_id!("SuspensionCollisionGrid::Cell");
        type Kind = cxx::kind::Opaque;
    }
    unsafe impl cxx::ExternType for bindgen::root::Vec {
        type Id = cxx::type_id!("Vec");
        type Kind = cxx::kind::Opaque;
    }
    unsafe impl cxx::ExternType for bindgen::root::btVehicleRaycaster_btVehicleRaycasterResult {
        type Id = cxx::type_id!("btVehicleRaycaster::btVehicleRaycasterResult");
        type Kind = cxx::kind::Opaque;
    }
    unsafe impl cxx::ExternType for bindgen::root::btCollisionObject {
        type Id = cxx::type_id!("btCollisionObject");
        type Kind = cxx::kind::Opaque;
    }
    mod bindgen {
        pub(super) mod root {
            #[repr(C, align(8))]
            pub struct SuspensionCollisionGrid {
                _pinned: core::marker::PhantomData<core::marker::PhantomPinned>,
                _non_send_sync: core::marker::PhantomData<[*const u8; 0]>,
                _data: [u8; 32],
            }
            #[repr(C, align(4))]
            pub struct SuspensionCollisionGrid_Cell {
                _pinned: core::marker::PhantomData<core::marker::PhantomPinned>,
                _non_send_sync: core::marker::PhantomData<[*const u8; 0]>,
                _data: [u8; 8],
            }
            #[repr(C, align(16))]
            pub struct Vec {
                _pinned: core::marker::PhantomData<core::marker::PhantomPinned>,
                _non_send_sync: core::marker::PhantomData<[*const u8; 0]>,
                _data: [u8; 16],
            }
            pub use cxxbridge::btVehicleRaycaster;
            #[repr(C, align(16))]
            pub struct btVehicleRaycaster_btVehicleRaycasterResult {
                _pinned: core::marker::PhantomData<core::marker::PhantomPinned>,
                _non_send_sync: core::marker::PhantomData<[*const u8; 0]>,
                _data: [u8; 48],
            }
            #[doc = " btCollisionObject can be used to manage collision detection objects.\n btCollisionObject maintains all information that is needed for a collision detection: Shape, Transform and AABB proxy.\n They can be added to the btCollisionWorld."]
            #[repr(C, align(16))]
            pub struct btCollisionObject {
                _pinned: core::marker::PhantomData<core::marker::PhantomPinned>,
                _non_send_sync: core::marker::PhantomData<[*const u8; 0]>,
                _data: [u8; 384],
            }
            impl SuspensionCollisionGrid {
                pub fn Get1<'a>(
                    self: &'a root::SuspensionCollisionGrid,
                    i: autocxx::c_int,
                    j: autocxx::c_int,
                    k: autocxx::c_int,
                ) -> impl autocxx::moveit::new::New<Output = root::SuspensionCollisionGrid_Cell> + 'a {
                    unsafe {
                        autocxx::moveit::new::by_raw(move |placement_return_type| {
                            let placement_return_type = placement_return_type.get_unchecked_mut().as_mut_ptr();
                            cxxbridge::Get1_autocxx_wrapper_0x224635b2cc063f4f(self, i, j, k, placement_return_type)
                        })
                    }
                }
                pub fn GetCellMin<'a>(
                    self: ::core::pin::Pin<&'a mut root::SuspensionCollisionGrid>,
                    xIndex: autocxx::c_int,
                    yIndex: autocxx::c_int,
                    zIndex: autocxx::c_int,
                ) -> impl autocxx::moveit::new::New<Output = root::Vec> + 'a {
                    unsafe {
                        autocxx::moveit::new::by_raw(move |placement_return_type| {
                            let placement_return_type = placement_return_type.get_unchecked_mut().as_mut_ptr();
                            cxxbridge::GetCellMin_autocxx_wrapper_0x224635b2cc063f4f(
                                self,
                                xIndex,
                                yIndex,
                                zIndex,
                                placement_return_type,
                            )
                        })
                    }
                }
                pub fn GetCellIndicesFromPos(
                    self: ::core::pin::Pin<&mut root::SuspensionCollisionGrid>,
                    pos: impl autocxx::ValueParam<root::Vec>,
                    i: ::core::pin::Pin<&mut autocxx::c_int>,
                    j: ::core::pin::Pin<&mut autocxx::c_int>,
                    k: ::core::pin::Pin<&mut autocxx::c_int>,
                ) {
                    let mut space0 = autocxx::ValueParamHandler::default();
                    let mut space0 = unsafe { ::core::pin::Pin::new_unchecked(&mut space0) };
                    unsafe {
                        space0.as_mut().populate(pos);
                        cxxbridge::GetCellIndicesFromPos_autocxx_wrapper_0x224635b2cc063f4f(self, space0.get_ptr(), i, j, k)
                    }
                }
                pub fn GetCellFromPos<'a>(
                    self: ::core::pin::Pin<&'a mut root::SuspensionCollisionGrid>,
                    pos: impl autocxx::ValueParam<root::Vec> + 'a,
                ) -> ::core::pin::Pin<&'a mut root::SuspensionCollisionGrid_Cell> {
                    let mut space0 = autocxx::ValueParamHandler::default();
                    let mut space0 = unsafe { ::core::pin::Pin::new_unchecked(&mut space0) };
                    unsafe {
                        space0.as_mut().populate(pos);
                        cxxbridge::GetCellFromPos_autocxx_wrapper_0x224635b2cc063f4f(self, space0.get_ptr())
                    }
                }
                pub fn GetCellSize<'a>(
                    self: ::core::pin::Pin<&'a mut root::SuspensionCollisionGrid>,
                ) -> impl autocxx::moveit::new::New<Output = root::Vec> + 'a {
                    unsafe {
                        autocxx::moveit::new::by_raw(move |placement_return_type| {
                            let placement_return_type = placement_return_type.get_unchecked_mut().as_mut_ptr();
                            cxxbridge::GetCellSize_autocxx_wrapper_0x224635b2cc063f4f(self, placement_return_type)
                        })
                    }
                }
                #[doc = "autocxx bindings couldn't be generated: Problem handling function argument triMeshShapes: Type std::vector was parameterized over something complex which we don't yet support"]
                fn SetupWorldCollision(_uhoh: autocxx::BindingGenerationFailure) {}
                pub unsafe fn CastSuspensionRay(
                    self: ::core::pin::Pin<&mut root::SuspensionCollisionGrid>,
                    raycaster: *mut root::btVehicleRaycaster,
                    start: impl autocxx::ValueParam<root::Vec>,
                    end: impl autocxx::ValueParam<root::Vec>,
                    result: ::core::pin::Pin<&mut root::btVehicleRaycaster_btVehicleRaycasterResult>,
                ) -> *mut root::btCollisionObject {
                    let mut space0 = autocxx::ValueParamHandler::default();
                    let mut space0 = ::core::pin::Pin::new_unchecked(&mut space0);
                    space0.as_mut().populate(start);
                    let mut space1 = autocxx::ValueParamHandler::default();
                    let mut space1 = ::core::pin::Pin::new_unchecked(&mut space1);
                    space1.as_mut().populate(end);
                    cxxbridge::CastSuspensionRay_autocxx_wrapper_0x224635b2cc063f4f(
                        self,
                        raycaster,
                        space0.get_ptr(),
                        space1.get_ptr(),
                        result,
                    )
                }
                pub fn UpdateDynamicCollisions(
                    self: ::core::pin::Pin<&mut root::SuspensionCollisionGrid>,
                    minBT: impl autocxx::ValueParam<root::Vec>,
                    maxBT: impl autocxx::ValueParam<root::Vec>,
                    remove: bool,
                ) {
                    let mut space0 = autocxx::ValueParamHandler::default();
                    let mut space0 = unsafe { ::core::pin::Pin::new_unchecked(&mut space0) };
                    unsafe {
                        space0.as_mut().populate(minBT);
                    }
                    let mut space1 = autocxx::ValueParamHandler::default();
                    let mut space1 = unsafe { ::core::pin::Pin::new_unchecked(&mut space1) };
                    unsafe {
                        space1.as_mut().populate(maxBT);
                        cxxbridge::UpdateDynamicCollisions_autocxx_wrapper_0x224635b2cc063f4f(
                            self,
                            space0.get_ptr(),
                            space1.get_ptr(),
                            remove,
                        )
                    }
                }
            }
            unsafe impl autocxx::moveit::MakeCppStorage for root::SuspensionCollisionGrid {
                unsafe fn allocate_uninitialized_cpp_storage() -> *mut root::SuspensionCollisionGrid {
                    cxxbridge::SuspensionCollisionGrid_alloc_autocxx_wrapper_0x224635b2cc063f4f()
                }
                unsafe fn free_uninitialized_cpp_storage(arg0: *mut root::SuspensionCollisionGrid) {
                    cxxbridge::SuspensionCollisionGrid_free_autocxx_wrapper_0x224635b2cc063f4f(arg0)
                }
            }
            unsafe impl autocxx::moveit::new::MoveNew for root::SuspensionCollisionGrid {
                #[doc = "Synthesized move constructor."]
                unsafe fn move_new(
                    mut other: ::core::pin::Pin<autocxx::moveit::MoveRef<'_, root::SuspensionCollisionGrid>>,
                    this: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<root::SuspensionCollisionGrid>>,
                ) {
                    cxxbridge :: SuspensionCollisionGrid_new_synthetic_move_ctor_0x224635b2cc063f4f_autocxx_wrapper_0x224635b2cc063f4f (this . get_unchecked_mut () . as_mut_ptr () , { let r : & mut _ = :: core :: pin :: Pin :: into_inner_unchecked (other . as_mut ()) ; r })
                }
            }
            impl Drop for root::SuspensionCollisionGrid {
                #[doc = "Synthesized destructor."]
                fn drop(self: &mut root::SuspensionCollisionGrid) {
                    unsafe {
                        cxxbridge :: SuspensionCollisionGrid_synthetic_destructor_0x224635b2cc063f4f_autocxx_wrapper_0x224635b2cc063f4f (self)
                    }
                }
            }
            unsafe impl autocxx::moveit::MakeCppStorage for root::SuspensionCollisionGrid_Cell {
                unsafe fn allocate_uninitialized_cpp_storage() -> *mut root::SuspensionCollisionGrid_Cell {
                    cxxbridge::SuspensionCollisionGrid_Cell_alloc_autocxx_wrapper_0x224635b2cc063f4f()
                }
                unsafe fn free_uninitialized_cpp_storage(arg0: *mut root::SuspensionCollisionGrid_Cell) {
                    cxxbridge::SuspensionCollisionGrid_Cell_free_autocxx_wrapper_0x224635b2cc063f4f(arg0)
                }
            }
            unsafe impl autocxx::moveit::new::MoveNew for root::SuspensionCollisionGrid_Cell {
                #[doc = "Synthesized move constructor."]
                unsafe fn move_new(
                    mut other: ::core::pin::Pin<autocxx::moveit::MoveRef<'_, root::SuspensionCollisionGrid_Cell>>,
                    this: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<root::SuspensionCollisionGrid_Cell>>,
                ) {
                    cxxbridge :: SuspensionCollisionGrid_Cell_synthetic_move_ctor_0x224635b2cc063f4f_autocxx_wrapper_0x224635b2cc063f4f (this . get_unchecked_mut () . as_mut_ptr () , { let r : & mut _ = :: core :: pin :: Pin :: into_inner_unchecked (other . as_mut ()) ; r })
                }
            }
            unsafe impl autocxx::moveit::new::CopyNew for root::SuspensionCollisionGrid_Cell {
                #[doc = "Synthesized copy constructor."]
                unsafe fn copy_new(
                    other: &root::SuspensionCollisionGrid_Cell,
                    this: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<root::SuspensionCollisionGrid_Cell>>,
                ) {
                    cxxbridge :: SuspensionCollisionGrid_Cell_synthetic_const_copy_ctor_0x224635b2cc063f4f_autocxx_wrapper_0x224635b2cc063f4f (this . get_unchecked_mut () . as_mut_ptr () , other)
                }
            }
            unsafe impl autocxx::moveit::MakeCppStorage for root::Vec {
                unsafe fn allocate_uninitialized_cpp_storage() -> *mut root::Vec {
                    cxxbridge::Vec_alloc_autocxx_wrapper_0x224635b2cc063f4f()
                }
                unsafe fn free_uninitialized_cpp_storage(arg0: *mut root::Vec) {
                    cxxbridge::Vec_free_autocxx_wrapper_0x224635b2cc063f4f(arg0)
                }
            }
            unsafe impl autocxx::moveit::new::MoveNew for root::Vec {
                #[doc = "Synthesized move constructor."]
                unsafe fn move_new(
                    mut other: ::core::pin::Pin<autocxx::moveit::MoveRef<'_, root::Vec>>,
                    this: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<root::Vec>>,
                ) {
                    cxxbridge::Vec_new_synthetic_move_ctor_0x224635b2cc063f4f_autocxx_wrapper_0x224635b2cc063f4f(
                        this.get_unchecked_mut().as_mut_ptr(),
                        {
                            let r: &mut _ = ::core::pin::Pin::into_inner_unchecked(other.as_mut());
                            r
                        },
                    )
                }
            }
            unsafe impl autocxx::moveit::new::CopyNew for root::Vec {
                #[doc = "Synthesized copy constructor."]
                unsafe fn copy_new(other: &root::Vec, this: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<root::Vec>>) {
                    cxxbridge::Vec_new_synthetic_const_copy_ctor_0x224635b2cc063f4f_autocxx_wrapper_0x224635b2cc063f4f(
                        this.get_unchecked_mut().as_mut_ptr(),
                        other,
                    )
                }
            }
            unsafe impl autocxx::moveit::MakeCppStorage for root::btVehicleRaycaster_btVehicleRaycasterResult {
                unsafe fn allocate_uninitialized_cpp_storage() -> *mut root::btVehicleRaycaster_btVehicleRaycasterResult {
                    cxxbridge::btVehicleRaycaster_btVehicleRaycasterResult_alloc_autocxx_wrapper_0x224635b2cc063f4f()
                }
                unsafe fn free_uninitialized_cpp_storage(arg0: *mut root::btVehicleRaycaster_btVehicleRaycasterResult) {
                    cxxbridge::btVehicleRaycaster_btVehicleRaycasterResult_free_autocxx_wrapper_0x224635b2cc063f4f(arg0)
                }
            }
            unsafe impl autocxx::moveit::new::CopyNew for root::btVehicleRaycaster_btVehicleRaycasterResult {
                #[doc = "Synthesized copy constructor."]
                unsafe fn copy_new(
                    other: &root::btVehicleRaycaster_btVehicleRaycasterResult,
                    this: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<root::btVehicleRaycaster_btVehicleRaycasterResult>>,
                ) {
                    cxxbridge :: btVehicleRaycaster_btVehicleRaycasterResult_synthetic_const_copy_ctor_0x224635b2cc063f4f_autocxx_wrapper_0x224635b2cc063f4f (this . get_unchecked_mut () . as_mut_ptr () , other)
                }
            }
            unsafe impl autocxx::moveit::MakeCppStorage for root::btCollisionObject {
                unsafe fn allocate_uninitialized_cpp_storage() -> *mut root::btCollisionObject {
                    cxxbridge::btCollisionObject_alloc_autocxx_wrapper_0x224635b2cc063f4f()
                }
                unsafe fn free_uninitialized_cpp_storage(arg0: *mut root::btCollisionObject) {
                    cxxbridge::btCollisionObject_free_autocxx_wrapper_0x224635b2cc063f4f(arg0)
                }
            }
            #[allow(unused_imports)]
            use self::super::super::{cxxbridge, ToCppString};
            #[allow(unused_imports)]
            use self::super::root;
        }
    }
    #[cxx::bridge]
    mod cxxbridge {
        impl UniquePtr<SuspensionCollisionGrid> {}
        impl SharedPtr<SuspensionCollisionGrid> {}
        impl WeakPtr<SuspensionCollisionGrid> {}
        impl CxxVector<SuspensionCollisionGrid> {}
        impl UniquePtr<SuspensionCollisionGrid_Cell> {}
        impl SharedPtr<SuspensionCollisionGrid_Cell> {}
        impl WeakPtr<SuspensionCollisionGrid_Cell> {}
        impl CxxVector<SuspensionCollisionGrid_Cell> {}
        impl UniquePtr<Vec> {}
        impl SharedPtr<Vec> {}
        impl WeakPtr<Vec> {}
        impl CxxVector<Vec> {}
        impl UniquePtr<btVehicleRaycaster> {}
        impl SharedPtr<btVehicleRaycaster> {}
        impl WeakPtr<btVehicleRaycaster> {}
        impl UniquePtr<btVehicleRaycaster_btVehicleRaycasterResult> {}
        impl SharedPtr<btVehicleRaycaster_btVehicleRaycasterResult> {}
        impl WeakPtr<btVehicleRaycaster_btVehicleRaycasterResult> {}
        impl UniquePtr<btCollisionObject> {}
        impl SharedPtr<btCollisionObject> {}
        impl WeakPtr<btCollisionObject> {}
        unsafe extern "C++" {
            fn autocxx_make_string_0x224635b2cc063f4f(str_: &str) -> UniquePtr<CxxString>;
            pub unsafe fn SuspensionCollisionGrid_alloc_autocxx_wrapper_0x224635b2cc063f4f() -> *mut SuspensionCollisionGrid;
            pub unsafe fn SuspensionCollisionGrid_free_autocxx_wrapper_0x224635b2cc063f4f(
                arg0: *mut SuspensionCollisionGrid,
            );
            type SuspensionCollisionGrid = super::bindgen::root::SuspensionCollisionGrid;
            pub fn Allocate(self: Pin<&mut SuspensionCollisionGrid>);
            pub fn Get<'a>(
                self: Pin<&'a mut SuspensionCollisionGrid>,
                i: c_int,
                j: c_int,
                k: c_int,
            ) -> Pin<&'a mut SuspensionCollisionGrid_Cell>;
            pub unsafe fn Get1_autocxx_wrapper_0x224635b2cc063f4f(
                autocxx_gen_this: &SuspensionCollisionGrid,
                i: c_int,
                j: c_int,
                k: c_int,
                placement_return_type: *mut SuspensionCollisionGrid_Cell,
            );
            pub unsafe fn GetCellMin_autocxx_wrapper_0x224635b2cc063f4f(
                autocxx_gen_this: Pin<&mut SuspensionCollisionGrid>,
                xIndex: c_int,
                yIndex: c_int,
                zIndex: c_int,
                placement_return_type: *mut Vec,
            );
            pub unsafe fn GetCellIndicesFromPos_autocxx_wrapper_0x224635b2cc063f4f(
                autocxx_gen_this: Pin<&mut SuspensionCollisionGrid>,
                pos: *mut Vec,
                i: Pin<&mut c_int>,
                j: Pin<&mut c_int>,
                k: Pin<&mut c_int>,
            );
            pub unsafe fn GetCellFromPos_autocxx_wrapper_0x224635b2cc063f4f<'a>(
                autocxx_gen_this: Pin<&'a mut SuspensionCollisionGrid>,
                pos: *mut Vec,
            ) -> Pin<&'a mut SuspensionCollisionGrid_Cell>;
            pub unsafe fn GetCellSize_autocxx_wrapper_0x224635b2cc063f4f(
                autocxx_gen_this: Pin<&mut SuspensionCollisionGrid>,
                placement_return_type: *mut Vec,
            );
            pub unsafe fn CastSuspensionRay_autocxx_wrapper_0x224635b2cc063f4f(
                autocxx_gen_this: Pin<&mut SuspensionCollisionGrid>,
                raycaster: *mut btVehicleRaycaster,
                start: *mut Vec,
                end: *mut Vec,
                result: Pin<&mut btVehicleRaycaster_btVehicleRaycasterResult>,
            ) -> *mut btCollisionObject;
            pub unsafe fn UpdateDynamicCollisions_autocxx_wrapper_0x224635b2cc063f4f(
                autocxx_gen_this: Pin<&mut SuspensionCollisionGrid>,
                minBT: *mut Vec,
                maxBT: *mut Vec,
                remove: bool,
            );
            #[doc = "Synthesized move constructor."]
            pub unsafe fn SuspensionCollisionGrid_new_synthetic_move_ctor_0x224635b2cc063f4f_autocxx_wrapper_0x224635b2cc063f4f(
                autocxx_gen_this: *mut SuspensionCollisionGrid,
                other: *mut SuspensionCollisionGrid,
            );
            #[doc = "Synthesized destructor."]
            pub unsafe fn SuspensionCollisionGrid_synthetic_destructor_0x224635b2cc063f4f_autocxx_wrapper_0x224635b2cc063f4f(
                autocxx_gen_this: *mut SuspensionCollisionGrid,
            );
            #[namespace = "SuspensionCollisionGrid"]
            #[cxx_name = "Cell"]
            type SuspensionCollisionGrid_Cell = super::bindgen::root::SuspensionCollisionGrid_Cell;
            type Vec = super::bindgen::root::Vec;
            #[doc = " btVehicleRaycaster is provides interface for between vehicle simulation and raycasting"]
            type btVehicleRaycaster;
            #[namespace = "btVehicleRaycaster"]
            #[cxx_name = "btVehicleRaycasterResult"]
            type btVehicleRaycaster_btVehicleRaycasterResult =
                super::bindgen::root::btVehicleRaycaster_btVehicleRaycasterResult;
            #[doc = " btCollisionObject can be used to manage collision detection objects.\n btCollisionObject maintains all information that is needed for a collision detection: Shape, Transform and AABB proxy.\n They can be added to the btCollisionWorld."]
            type btCollisionObject = super::bindgen::root::btCollisionObject;
            pub unsafe fn SuspensionCollisionGrid_Cell_alloc_autocxx_wrapper_0x224635b2cc063f4f(
            ) -> *mut SuspensionCollisionGrid_Cell;
            pub unsafe fn SuspensionCollisionGrid_Cell_free_autocxx_wrapper_0x224635b2cc063f4f(
                arg0: *mut SuspensionCollisionGrid_Cell,
            );
            #[doc = "Synthesized move constructor."]
            pub unsafe fn SuspensionCollisionGrid_Cell_synthetic_move_ctor_0x224635b2cc063f4f_autocxx_wrapper_0x224635b2cc063f4f(
                autocxx_gen_this: *mut SuspensionCollisionGrid_Cell,
                other: *mut SuspensionCollisionGrid_Cell,
            );
            #[doc = "Synthesized copy constructor."]
            pub unsafe fn SuspensionCollisionGrid_Cell_synthetic_const_copy_ctor_0x224635b2cc063f4f_autocxx_wrapper_0x224635b2cc063f4f(
                autocxx_gen_this: *mut SuspensionCollisionGrid_Cell,
                other: &SuspensionCollisionGrid_Cell,
            );
            pub unsafe fn Vec_alloc_autocxx_wrapper_0x224635b2cc063f4f() -> *mut Vec;
            pub unsafe fn Vec_free_autocxx_wrapper_0x224635b2cc063f4f(arg0: *mut Vec);
            #[doc = "Synthesized move constructor."]
            pub unsafe fn Vec_new_synthetic_move_ctor_0x224635b2cc063f4f_autocxx_wrapper_0x224635b2cc063f4f(
                autocxx_gen_this: *mut Vec,
                other: *mut Vec,
            );
            #[doc = "Synthesized copy constructor."]
            pub unsafe fn Vec_new_synthetic_const_copy_ctor_0x224635b2cc063f4f_autocxx_wrapper_0x224635b2cc063f4f(
                autocxx_gen_this: *mut Vec,
                other: &Vec,
            );
            pub unsafe fn btVehicleRaycaster_btVehicleRaycasterResult_alloc_autocxx_wrapper_0x224635b2cc063f4f(
            ) -> *mut btVehicleRaycaster_btVehicleRaycasterResult;
            pub unsafe fn btVehicleRaycaster_btVehicleRaycasterResult_free_autocxx_wrapper_0x224635b2cc063f4f(
                arg0: *mut btVehicleRaycaster_btVehicleRaycasterResult,
            );
            #[doc = "Synthesized copy constructor."]
            pub unsafe fn btVehicleRaycaster_btVehicleRaycasterResult_synthetic_const_copy_ctor_0x224635b2cc063f4f_autocxx_wrapper_0x224635b2cc063f4f(
                autocxx_gen_this: *mut btVehicleRaycaster_btVehicleRaycasterResult,
                other: &btVehicleRaycaster_btVehicleRaycasterResult,
            );
            pub unsafe fn btCollisionObject_alloc_autocxx_wrapper_0x224635b2cc063f4f() -> *mut btCollisionObject;
            pub unsafe fn btCollisionObject_free_autocxx_wrapper_0x224635b2cc063f4f(arg0: *mut btCollisionObject);
            type c_int = autocxx::c_int;
            include!("Sim/SuspensionCollisionGrid/SuspensionCollisionGrid.h");
            include!("autocxxgen_scg.h");
        }
        extern "Rust" {}
    }
    #[allow(unused_imports)]
    use bindgen::root;
    pub use bindgen::root::btCollisionObject;
    pub use bindgen::root::btVehicleRaycaster;
    pub use bindgen::root::btVehicleRaycaster_btVehicleRaycasterResult;
    pub use bindgen::root::SuspensionCollisionGrid;
    pub use bindgen::root::SuspensionCollisionGrid_Cell;
    pub use bindgen::root::Vec;
    pub use cxxbridge::autocxx_make_string_0x224635b2cc063f4f as make_string;
}
