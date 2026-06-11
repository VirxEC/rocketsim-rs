pub use root::*;

const _: () = ::planus::check_version_compatibility("planus-1.3.0");

/// The root namespace
///
/// Generated from these locations:
/// * File `spec/core.fbs`
#[allow(clippy::needless_lifetimes)]
mod root {
    /// The namespace `rocketsim`
    ///
    /// Generated from these locations:
    /// * File `spec/core.fbs`
    /// * File `spec/game_state.fbs`
    /// * File `spec/render.fbs`
    /// * File `spec/common.fbs`
    pub mod rocketsim {
        /// The table `Connection` in the namespace `rocketsim`
        ///
        /// Generated from these locations:
        /// * Table `Connection` in the file `spec/core.fbs:6`
        #[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, ::serde::Serialize, ::serde::Deserialize)]
        pub struct Connection {}

        #[allow(clippy::derivable_impls)]
        impl ::core::default::Default for Connection {
            fn default() -> Self {
                Self {}
            }
        }

        impl Connection {
            /// Creates a [ConnectionBuilder] for serializing an instance of this table.
            #[inline]
            pub fn builder() -> ConnectionBuilder<()> {
                ConnectionBuilder(())
            }

            #[allow(clippy::too_many_arguments)]
            pub fn create(builder: &mut ::planus::Builder) -> ::planus::Offset<Self> {
                let table_writer: ::planus::table_writer::TableWriter<4> = ::core::default::Default::default();
                unsafe {
                    table_writer.finish(builder, |_table_writer| {});
                }
                builder.current_offset()
            }
        }

        impl ::planus::WriteAs<::planus::Offset<Connection>> for Connection {
            type Prepared = ::planus::Offset<Self>;

            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<Connection> {
                ::planus::WriteAsOffset::prepare(self, builder)
            }
        }

        impl ::planus::WriteAsOptional<::planus::Offset<Connection>> for Connection {
            type Prepared = ::planus::Offset<Self>;

            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::core::option::Option<::planus::Offset<Connection>> {
                ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
            }
        }

        impl ::planus::WriteAsOffset<Connection> for Connection {
            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<Connection> {
                Connection::create(builder)
            }
        }

        /// Builder for serializing an instance of the [Connection] type.
        ///
        /// Can be created using the [Connection::builder] method.
        #[derive(Debug)]
        #[must_use]
        pub struct ConnectionBuilder<State>(State);

        impl ConnectionBuilder<()> {
            /// Finish writing the builder to get an [Offset](::planus::Offset) to a serialized [Connection].
            #[inline]
            pub fn finish(self, builder: &mut ::planus::Builder) -> ::planus::Offset<Connection>
            where
                Self: ::planus::WriteAsOffset<Connection>,
            {
                ::planus::WriteAsOffset::prepare(&self, builder)
            }
        }

        impl ::planus::WriteAs<::planus::Offset<Connection>> for ConnectionBuilder<()> {
            type Prepared = ::planus::Offset<Connection>;

            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<Connection> {
                ::planus::WriteAsOffset::prepare(self, builder)
            }
        }

        impl ::planus::WriteAsOptional<::planus::Offset<Connection>> for ConnectionBuilder<()> {
            type Prepared = ::planus::Offset<Connection>;

            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::core::option::Option<::planus::Offset<Connection>> {
                ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
            }
        }

        impl ::planus::WriteAsOffset<Connection> for ConnectionBuilder<()> {
            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<Connection> {
                Connection::create(builder)
            }
        }

        /// Reference to a deserialized [Connection].
        #[derive(Copy, Clone)]
        pub struct ConnectionRef<'a>(#[allow(dead_code)] ::planus::table_reader::Table<'a>);

        impl<'a> ConnectionRef<'a> {}

        impl<'a> ::core::fmt::Debug for ConnectionRef<'a> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                let mut f = f.debug_struct("ConnectionRef");

                f.finish()
            }
        }

        impl<'a> ::core::convert::TryFrom<ConnectionRef<'a>> for Connection {
            type Error = ::planus::Error;

            fn try_from(_value: ConnectionRef<'a>) -> ::planus::Result<Self> {
                ::core::result::Result::Ok(Self {})
            }
        }

        impl<'a> ::planus::TableRead<'a> for ConnectionRef<'a> {
            #[inline]
            fn from_buffer(
                buffer: ::planus::SliceWithStartOffset<'a>,
                offset: usize,
            ) -> ::core::result::Result<Self, ::planus::errors::ErrorKind> {
                ::core::result::Result::Ok(Self(::planus::table_reader::Table::from_buffer(buffer, offset)?))
            }
        }

        impl<'a> ::planus::VectorReadInner<'a> for ConnectionRef<'a> {
            type Error = ::planus::Error;
            const STRIDE: usize = 4;

            unsafe fn from_buffer(buffer: ::planus::SliceWithStartOffset<'a>, offset: usize) -> ::planus::Result<Self> {
                ::planus::TableRead::from_buffer(buffer, offset)
                    .map_err(|error_kind| error_kind.with_error_location("[ConnectionRef]", "get", buffer.offset_from_start))
            }
        }

        /// # Safety
        /// The planus compiler generates implementations that initialize
        /// the bytes in `write_values`.
        unsafe impl ::planus::VectorWrite<::planus::Offset<Connection>> for Connection {
            type Value = ::planus::Offset<Connection>;
            const STRIDE: usize = 4;
            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> Self::Value {
                ::planus::WriteAs::prepare(self, builder)
            }

            #[inline]
            unsafe fn write_values(
                values: &[::planus::Offset<Connection>],
                bytes: *mut ::core::mem::MaybeUninit<u8>,
                buffer_position: u32,
            ) {
                let bytes = bytes as *mut [::core::mem::MaybeUninit<u8>; 4];
                for (i, v) in ::core::iter::Iterator::enumerate(values.iter()) {
                    ::planus::WriteAsPrimitive::write(
                        v,
                        ::planus::Cursor::new(unsafe { &mut *bytes.add(i) }),
                        buffer_position - (Self::STRIDE * i) as u32,
                    );
                }
            }
        }

        impl<'a> ::planus::ReadAsRoot<'a> for ConnectionRef<'a> {
            fn read_as_root(slice: &'a [u8]) -> ::planus::Result<Self> {
                ::planus::TableRead::from_buffer(
                    ::planus::SliceWithStartOffset {
                        buffer: slice,
                        offset_from_start: 0,
                    },
                    0,
                )
                .map_err(|error_kind| error_kind.with_error_location("[ConnectionRef]", "read_as_root", 0))
            }
        }

        /// The table `Quit` in the namespace `rocketsim`
        ///
        /// Generated from these locations:
        /// * Table `Quit` in the file `spec/core.fbs:8`
        #[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, ::serde::Serialize, ::serde::Deserialize)]
        pub struct Quit {}

        #[allow(clippy::derivable_impls)]
        impl ::core::default::Default for Quit {
            fn default() -> Self {
                Self {}
            }
        }

        impl Quit {
            /// Creates a [QuitBuilder] for serializing an instance of this table.
            #[inline]
            pub fn builder() -> QuitBuilder<()> {
                QuitBuilder(())
            }

            #[allow(clippy::too_many_arguments)]
            pub fn create(builder: &mut ::planus::Builder) -> ::planus::Offset<Self> {
                let table_writer: ::planus::table_writer::TableWriter<4> = ::core::default::Default::default();
                unsafe {
                    table_writer.finish(builder, |_table_writer| {});
                }
                builder.current_offset()
            }
        }

        impl ::planus::WriteAs<::planus::Offset<Quit>> for Quit {
            type Prepared = ::planus::Offset<Self>;

            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<Quit> {
                ::planus::WriteAsOffset::prepare(self, builder)
            }
        }

        impl ::planus::WriteAsOptional<::planus::Offset<Quit>> for Quit {
            type Prepared = ::planus::Offset<Self>;

            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::core::option::Option<::planus::Offset<Quit>> {
                ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
            }
        }

        impl ::planus::WriteAsOffset<Quit> for Quit {
            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<Quit> {
                Quit::create(builder)
            }
        }

        /// Builder for serializing an instance of the [Quit] type.
        ///
        /// Can be created using the [Quit::builder] method.
        #[derive(Debug)]
        #[must_use]
        pub struct QuitBuilder<State>(State);

        impl QuitBuilder<()> {
            /// Finish writing the builder to get an [Offset](::planus::Offset) to a serialized [Quit].
            #[inline]
            pub fn finish(self, builder: &mut ::planus::Builder) -> ::planus::Offset<Quit>
            where
                Self: ::planus::WriteAsOffset<Quit>,
            {
                ::planus::WriteAsOffset::prepare(&self, builder)
            }
        }

        impl ::planus::WriteAs<::planus::Offset<Quit>> for QuitBuilder<()> {
            type Prepared = ::planus::Offset<Quit>;

            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<Quit> {
                ::planus::WriteAsOffset::prepare(self, builder)
            }
        }

        impl ::planus::WriteAsOptional<::planus::Offset<Quit>> for QuitBuilder<()> {
            type Prepared = ::planus::Offset<Quit>;

            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::core::option::Option<::planus::Offset<Quit>> {
                ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
            }
        }

        impl ::planus::WriteAsOffset<Quit> for QuitBuilder<()> {
            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<Quit> {
                Quit::create(builder)
            }
        }

        /// Reference to a deserialized [Quit].
        #[derive(Copy, Clone)]
        pub struct QuitRef<'a>(#[allow(dead_code)] ::planus::table_reader::Table<'a>);

        impl<'a> QuitRef<'a> {}

        impl<'a> ::core::fmt::Debug for QuitRef<'a> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                let mut f = f.debug_struct("QuitRef");

                f.finish()
            }
        }

        impl<'a> ::core::convert::TryFrom<QuitRef<'a>> for Quit {
            type Error = ::planus::Error;

            fn try_from(_value: QuitRef<'a>) -> ::planus::Result<Self> {
                ::core::result::Result::Ok(Self {})
            }
        }

        impl<'a> ::planus::TableRead<'a> for QuitRef<'a> {
            #[inline]
            fn from_buffer(
                buffer: ::planus::SliceWithStartOffset<'a>,
                offset: usize,
            ) -> ::core::result::Result<Self, ::planus::errors::ErrorKind> {
                ::core::result::Result::Ok(Self(::planus::table_reader::Table::from_buffer(buffer, offset)?))
            }
        }

        impl<'a> ::planus::VectorReadInner<'a> for QuitRef<'a> {
            type Error = ::planus::Error;
            const STRIDE: usize = 4;

            unsafe fn from_buffer(buffer: ::planus::SliceWithStartOffset<'a>, offset: usize) -> ::planus::Result<Self> {
                ::planus::TableRead::from_buffer(buffer, offset)
                    .map_err(|error_kind| error_kind.with_error_location("[QuitRef]", "get", buffer.offset_from_start))
            }
        }

        /// # Safety
        /// The planus compiler generates implementations that initialize
        /// the bytes in `write_values`.
        unsafe impl ::planus::VectorWrite<::planus::Offset<Quit>> for Quit {
            type Value = ::planus::Offset<Quit>;
            const STRIDE: usize = 4;
            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> Self::Value {
                ::planus::WriteAs::prepare(self, builder)
            }

            #[inline]
            unsafe fn write_values(
                values: &[::planus::Offset<Quit>],
                bytes: *mut ::core::mem::MaybeUninit<u8>,
                buffer_position: u32,
            ) {
                let bytes = bytes as *mut [::core::mem::MaybeUninit<u8>; 4];
                for (i, v) in ::core::iter::Iterator::enumerate(values.iter()) {
                    ::planus::WriteAsPrimitive::write(
                        v,
                        ::planus::Cursor::new(unsafe { &mut *bytes.add(i) }),
                        buffer_position - (Self::STRIDE * i) as u32,
                    );
                }
            }
        }

        impl<'a> ::planus::ReadAsRoot<'a> for QuitRef<'a> {
            fn read_as_root(slice: &'a [u8]) -> ::planus::Result<Self> {
                ::planus::TableRead::from_buffer(
                    ::planus::SliceWithStartOffset {
                        buffer: slice,
                        offset_from_start: 0,
                    },
                    0,
                )
                .map_err(|error_kind| error_kind.with_error_location("[QuitRef]", "read_as_root", 0))
            }
        }

        /// The table `Speed` in the namespace `rocketsim`
        ///
        /// Generated from these locations:
        /// * Table `Speed` in the file `spec/core.fbs:10`
        #[derive(Clone, Debug, PartialEq, PartialOrd, ::serde::Serialize, ::serde::Deserialize)]
        pub struct Speed {
            /// The field `speed` in the table `Speed`
            pub speed: f32,
        }

        #[allow(clippy::derivable_impls)]
        impl ::core::default::Default for Speed {
            fn default() -> Self {
                Self { speed: 0.0 }
            }
        }

        impl Speed {
            /// Creates a [SpeedBuilder] for serializing an instance of this table.
            #[inline]
            pub fn builder() -> SpeedBuilder<()> {
                SpeedBuilder(())
            }

            #[allow(clippy::too_many_arguments)]
            pub fn create(
                builder: &mut ::planus::Builder,
                field_speed: impl ::planus::WriteAsDefault<f32, f32>,
            ) -> ::planus::Offset<Self> {
                let prepared_speed = field_speed.prepare(builder, &0.0);

                let mut table_writer: ::planus::table_writer::TableWriter<6> = ::core::default::Default::default();
                if prepared_speed.is_some() {
                    table_writer.write_entry::<f32>(0);
                }

                unsafe {
                    table_writer.finish(builder, |object_writer| {
                        if let ::core::option::Option::Some(prepared_speed) = prepared_speed {
                            object_writer.write::<_, _, 4>(&prepared_speed);
                        }
                    });
                }
                builder.current_offset()
            }
        }

        impl ::planus::WriteAs<::planus::Offset<Speed>> for Speed {
            type Prepared = ::planus::Offset<Self>;

            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<Speed> {
                ::planus::WriteAsOffset::prepare(self, builder)
            }
        }

        impl ::planus::WriteAsOptional<::planus::Offset<Speed>> for Speed {
            type Prepared = ::planus::Offset<Self>;

            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::core::option::Option<::planus::Offset<Speed>> {
                ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
            }
        }

        impl ::planus::WriteAsOffset<Speed> for Speed {
            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<Speed> {
                Speed::create(builder, self.speed)
            }
        }

        /// Builder for serializing an instance of the [Speed] type.
        ///
        /// Can be created using the [Speed::builder] method.
        #[derive(Debug)]
        #[must_use]
        pub struct SpeedBuilder<State>(State);

        impl SpeedBuilder<()> {
            /// Setter for the [`speed` field](Speed#structfield.speed).
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn speed<T0>(self, value: T0) -> SpeedBuilder<(T0,)>
            where
                T0: ::planus::WriteAsDefault<f32, f32>,
            {
                SpeedBuilder((value,))
            }

            /// Sets the [`speed` field](Speed#structfield.speed) to the default value.
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn speed_as_default(self) -> SpeedBuilder<(::planus::DefaultValue,)> {
                self.speed(::planus::DefaultValue)
            }
        }

        impl<T0> SpeedBuilder<(T0,)> {
            /// Finish writing the builder to get an [Offset](::planus::Offset) to a serialized [Speed].
            #[inline]
            pub fn finish(self, builder: &mut ::planus::Builder) -> ::planus::Offset<Speed>
            where
                Self: ::planus::WriteAsOffset<Speed>,
            {
                ::planus::WriteAsOffset::prepare(&self, builder)
            }
        }

        impl<T0: ::planus::WriteAsDefault<f32, f32>> ::planus::WriteAs<::planus::Offset<Speed>> for SpeedBuilder<(T0,)> {
            type Prepared = ::planus::Offset<Speed>;

            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<Speed> {
                ::planus::WriteAsOffset::prepare(self, builder)
            }
        }

        impl<T0: ::planus::WriteAsDefault<f32, f32>> ::planus::WriteAsOptional<::planus::Offset<Speed>> for SpeedBuilder<(T0,)> {
            type Prepared = ::planus::Offset<Speed>;

            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::core::option::Option<::planus::Offset<Speed>> {
                ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
            }
        }

        impl<T0: ::planus::WriteAsDefault<f32, f32>> ::planus::WriteAsOffset<Speed> for SpeedBuilder<(T0,)> {
            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<Speed> {
                let (v0,) = &self.0;
                Speed::create(builder, v0)
            }
        }

        /// Reference to a deserialized [Speed].
        #[derive(Copy, Clone)]
        pub struct SpeedRef<'a>(#[allow(dead_code)] ::planus::table_reader::Table<'a>);

        impl<'a> SpeedRef<'a> {
            /// Getter for the [`speed` field](Speed#structfield.speed).
            #[inline]
            pub fn speed(&self) -> ::planus::Result<f32> {
                ::core::result::Result::Ok(self.0.access(0, "Speed", "speed")?.unwrap_or(0.0))
            }
        }

        impl<'a> ::core::fmt::Debug for SpeedRef<'a> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                let mut f = f.debug_struct("SpeedRef");
                f.field("speed", &self.speed());
                f.finish()
            }
        }

        impl<'a> ::core::convert::TryFrom<SpeedRef<'a>> for Speed {
            type Error = ::planus::Error;

            #[allow(unreachable_code)]
            fn try_from(value: SpeedRef<'a>) -> ::planus::Result<Self> {
                ::core::result::Result::Ok(Self {
                    speed: ::core::convert::TryInto::try_into(value.speed()?)?,
                })
            }
        }

        impl<'a> ::planus::TableRead<'a> for SpeedRef<'a> {
            #[inline]
            fn from_buffer(
                buffer: ::planus::SliceWithStartOffset<'a>,
                offset: usize,
            ) -> ::core::result::Result<Self, ::planus::errors::ErrorKind> {
                ::core::result::Result::Ok(Self(::planus::table_reader::Table::from_buffer(buffer, offset)?))
            }
        }

        impl<'a> ::planus::VectorReadInner<'a> for SpeedRef<'a> {
            type Error = ::planus::Error;
            const STRIDE: usize = 4;

            unsafe fn from_buffer(buffer: ::planus::SliceWithStartOffset<'a>, offset: usize) -> ::planus::Result<Self> {
                ::planus::TableRead::from_buffer(buffer, offset)
                    .map_err(|error_kind| error_kind.with_error_location("[SpeedRef]", "get", buffer.offset_from_start))
            }
        }

        /// # Safety
        /// The planus compiler generates implementations that initialize
        /// the bytes in `write_values`.
        unsafe impl ::planus::VectorWrite<::planus::Offset<Speed>> for Speed {
            type Value = ::planus::Offset<Speed>;
            const STRIDE: usize = 4;
            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> Self::Value {
                ::planus::WriteAs::prepare(self, builder)
            }

            #[inline]
            unsafe fn write_values(
                values: &[::planus::Offset<Speed>],
                bytes: *mut ::core::mem::MaybeUninit<u8>,
                buffer_position: u32,
            ) {
                let bytes = bytes as *mut [::core::mem::MaybeUninit<u8>; 4];
                for (i, v) in ::core::iter::Iterator::enumerate(values.iter()) {
                    ::planus::WriteAsPrimitive::write(
                        v,
                        ::planus::Cursor::new(unsafe { &mut *bytes.add(i) }),
                        buffer_position - (Self::STRIDE * i) as u32,
                    );
                }
            }
        }

        impl<'a> ::planus::ReadAsRoot<'a> for SpeedRef<'a> {
            fn read_as_root(slice: &'a [u8]) -> ::planus::Result<Self> {
                ::planus::TableRead::from_buffer(
                    ::planus::SliceWithStartOffset {
                        buffer: slice,
                        offset_from_start: 0,
                    },
                    0,
                )
                .map_err(|error_kind| error_kind.with_error_location("[SpeedRef]", "read_as_root", 0))
            }
        }

        /// The table `Paused` in the namespace `rocketsim`
        ///
        /// Generated from these locations:
        /// * Table `Paused` in the file `spec/core.fbs:14`
        #[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, ::serde::Serialize, ::serde::Deserialize)]
        pub struct Paused {
            /// The field `paused` in the table `Paused`
            pub paused: bool,
        }

        #[allow(clippy::derivable_impls)]
        impl ::core::default::Default for Paused {
            fn default() -> Self {
                Self { paused: false }
            }
        }

        impl Paused {
            /// Creates a [PausedBuilder] for serializing an instance of this table.
            #[inline]
            pub fn builder() -> PausedBuilder<()> {
                PausedBuilder(())
            }

            #[allow(clippy::too_many_arguments)]
            pub fn create(
                builder: &mut ::planus::Builder,
                field_paused: impl ::planus::WriteAsDefault<bool, bool>,
            ) -> ::planus::Offset<Self> {
                let prepared_paused = field_paused.prepare(builder, &false);

                let mut table_writer: ::planus::table_writer::TableWriter<6> = ::core::default::Default::default();
                if prepared_paused.is_some() {
                    table_writer.write_entry::<bool>(0);
                }

                unsafe {
                    table_writer.finish(builder, |object_writer| {
                        if let ::core::option::Option::Some(prepared_paused) = prepared_paused {
                            object_writer.write::<_, _, 1>(&prepared_paused);
                        }
                    });
                }
                builder.current_offset()
            }
        }

        impl ::planus::WriteAs<::planus::Offset<Paused>> for Paused {
            type Prepared = ::planus::Offset<Self>;

            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<Paused> {
                ::planus::WriteAsOffset::prepare(self, builder)
            }
        }

        impl ::planus::WriteAsOptional<::planus::Offset<Paused>> for Paused {
            type Prepared = ::planus::Offset<Self>;

            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::core::option::Option<::planus::Offset<Paused>> {
                ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
            }
        }

        impl ::planus::WriteAsOffset<Paused> for Paused {
            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<Paused> {
                Paused::create(builder, self.paused)
            }
        }

        /// Builder for serializing an instance of the [Paused] type.
        ///
        /// Can be created using the [Paused::builder] method.
        #[derive(Debug)]
        #[must_use]
        pub struct PausedBuilder<State>(State);

        impl PausedBuilder<()> {
            /// Setter for the [`paused` field](Paused#structfield.paused).
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn paused<T0>(self, value: T0) -> PausedBuilder<(T0,)>
            where
                T0: ::planus::WriteAsDefault<bool, bool>,
            {
                PausedBuilder((value,))
            }

            /// Sets the [`paused` field](Paused#structfield.paused) to the default value.
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn paused_as_default(self) -> PausedBuilder<(::planus::DefaultValue,)> {
                self.paused(::planus::DefaultValue)
            }
        }

        impl<T0> PausedBuilder<(T0,)> {
            /// Finish writing the builder to get an [Offset](::planus::Offset) to a serialized [Paused].
            #[inline]
            pub fn finish(self, builder: &mut ::planus::Builder) -> ::planus::Offset<Paused>
            where
                Self: ::planus::WriteAsOffset<Paused>,
            {
                ::planus::WriteAsOffset::prepare(&self, builder)
            }
        }

        impl<T0: ::planus::WriteAsDefault<bool, bool>> ::planus::WriteAs<::planus::Offset<Paused>> for PausedBuilder<(T0,)> {
            type Prepared = ::planus::Offset<Paused>;

            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<Paused> {
                ::planus::WriteAsOffset::prepare(self, builder)
            }
        }

        impl<T0: ::planus::WriteAsDefault<bool, bool>> ::planus::WriteAsOptional<::planus::Offset<Paused>> for PausedBuilder<(T0,)> {
            type Prepared = ::planus::Offset<Paused>;

            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::core::option::Option<::planus::Offset<Paused>> {
                ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
            }
        }

        impl<T0: ::planus::WriteAsDefault<bool, bool>> ::planus::WriteAsOffset<Paused> for PausedBuilder<(T0,)> {
            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<Paused> {
                let (v0,) = &self.0;
                Paused::create(builder, v0)
            }
        }

        /// Reference to a deserialized [Paused].
        #[derive(Copy, Clone)]
        pub struct PausedRef<'a>(#[allow(dead_code)] ::planus::table_reader::Table<'a>);

        impl<'a> PausedRef<'a> {
            /// Getter for the [`paused` field](Paused#structfield.paused).
            #[inline]
            pub fn paused(&self) -> ::planus::Result<bool> {
                ::core::result::Result::Ok(self.0.access(0, "Paused", "paused")?.unwrap_or(false))
            }
        }

        impl<'a> ::core::fmt::Debug for PausedRef<'a> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                let mut f = f.debug_struct("PausedRef");
                f.field("paused", &self.paused());
                f.finish()
            }
        }

        impl<'a> ::core::convert::TryFrom<PausedRef<'a>> for Paused {
            type Error = ::planus::Error;

            #[allow(unreachable_code)]
            fn try_from(value: PausedRef<'a>) -> ::planus::Result<Self> {
                ::core::result::Result::Ok(Self {
                    paused: ::core::convert::TryInto::try_into(value.paused()?)?,
                })
            }
        }

        impl<'a> ::planus::TableRead<'a> for PausedRef<'a> {
            #[inline]
            fn from_buffer(
                buffer: ::planus::SliceWithStartOffset<'a>,
                offset: usize,
            ) -> ::core::result::Result<Self, ::planus::errors::ErrorKind> {
                ::core::result::Result::Ok(Self(::planus::table_reader::Table::from_buffer(buffer, offset)?))
            }
        }

        impl<'a> ::planus::VectorReadInner<'a> for PausedRef<'a> {
            type Error = ::planus::Error;
            const STRIDE: usize = 4;

            unsafe fn from_buffer(buffer: ::planus::SliceWithStartOffset<'a>, offset: usize) -> ::planus::Result<Self> {
                ::planus::TableRead::from_buffer(buffer, offset)
                    .map_err(|error_kind| error_kind.with_error_location("[PausedRef]", "get", buffer.offset_from_start))
            }
        }

        /// # Safety
        /// The planus compiler generates implementations that initialize
        /// the bytes in `write_values`.
        unsafe impl ::planus::VectorWrite<::planus::Offset<Paused>> for Paused {
            type Value = ::planus::Offset<Paused>;
            const STRIDE: usize = 4;
            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> Self::Value {
                ::planus::WriteAs::prepare(self, builder)
            }

            #[inline]
            unsafe fn write_values(
                values: &[::planus::Offset<Paused>],
                bytes: *mut ::core::mem::MaybeUninit<u8>,
                buffer_position: u32,
            ) {
                let bytes = bytes as *mut [::core::mem::MaybeUninit<u8>; 4];
                for (i, v) in ::core::iter::Iterator::enumerate(values.iter()) {
                    ::planus::WriteAsPrimitive::write(
                        v,
                        ::planus::Cursor::new(unsafe { &mut *bytes.add(i) }),
                        buffer_position - (Self::STRIDE * i) as u32,
                    );
                }
            }
        }

        impl<'a> ::planus::ReadAsRoot<'a> for PausedRef<'a> {
            fn read_as_root(slice: &'a [u8]) -> ::planus::Result<Self> {
                ::planus::TableRead::from_buffer(
                    ::planus::SliceWithStartOffset {
                        buffer: slice,
                        offset_from_start: 0,
                    },
                    0,
                )
                .map_err(|error_kind| error_kind.with_error_location("[PausedRef]", "read_as_root", 0))
            }
        }

        /// The union `Message` in the namespace `rocketsim`
        ///
        /// Generated from these locations:
        /// * Union `Message` in the file `spec/core.fbs:18`
        #[derive(Clone, Debug, PartialEq, PartialOrd, ::serde::Serialize, ::serde::Deserialize)]
        pub enum Message {
            /// The variant of type `Connection` in the union `Message`
            Connection(::planus::alloc::boxed::Box<self::Connection>),

            /// The variant of type `Quit` in the union `Message`
            Quit(::planus::alloc::boxed::Box<self::Quit>),

            /// The variant of type `Speed` in the union `Message`
            Speed(::planus::alloc::boxed::Box<self::Speed>),

            /// The variant of type `Paused` in the union `Message`
            Paused(::planus::alloc::boxed::Box<self::Paused>),

            /// The variant of type `GameState` in the union `Message`
            GameState(::planus::alloc::boxed::Box<self::GameState>),

            /// The variant of type `AddRender` in the union `Message`
            AddRender(::planus::alloc::boxed::Box<self::AddRender>),

            /// The variant of type `RemoveRender` in the union `Message`
            RemoveRender(::planus::alloc::boxed::Box<self::RemoveRender>),
        }

        impl Message {
            /// Creates a [MessageBuilder] for serializing an instance of this table.
            #[inline]
            pub fn builder() -> MessageBuilder<::planus::Uninitialized> {
                MessageBuilder(::planus::Uninitialized)
            }

            #[inline]
            pub fn create_connection(
                builder: &mut ::planus::Builder,
                value: impl ::planus::WriteAsOffset<self::Connection>,
            ) -> ::planus::UnionOffset<Self> {
                ::planus::UnionOffset::new(1, value.prepare(builder).downcast())
            }

            #[inline]
            pub fn create_quit(
                builder: &mut ::planus::Builder,
                value: impl ::planus::WriteAsOffset<self::Quit>,
            ) -> ::planus::UnionOffset<Self> {
                ::planus::UnionOffset::new(2, value.prepare(builder).downcast())
            }

            #[inline]
            pub fn create_speed(
                builder: &mut ::planus::Builder,
                value: impl ::planus::WriteAsOffset<self::Speed>,
            ) -> ::planus::UnionOffset<Self> {
                ::planus::UnionOffset::new(3, value.prepare(builder).downcast())
            }

            #[inline]
            pub fn create_paused(
                builder: &mut ::planus::Builder,
                value: impl ::planus::WriteAsOffset<self::Paused>,
            ) -> ::planus::UnionOffset<Self> {
                ::planus::UnionOffset::new(4, value.prepare(builder).downcast())
            }

            #[inline]
            pub fn create_game_state(
                builder: &mut ::planus::Builder,
                value: impl ::planus::WriteAsOffset<self::GameState>,
            ) -> ::planus::UnionOffset<Self> {
                ::planus::UnionOffset::new(5, value.prepare(builder).downcast())
            }

            #[inline]
            pub fn create_add_render(
                builder: &mut ::planus::Builder,
                value: impl ::planus::WriteAsOffset<self::AddRender>,
            ) -> ::planus::UnionOffset<Self> {
                ::planus::UnionOffset::new(6, value.prepare(builder).downcast())
            }

            #[inline]
            pub fn create_remove_render(
                builder: &mut ::planus::Builder,
                value: impl ::planus::WriteAsOffset<self::RemoveRender>,
            ) -> ::planus::UnionOffset<Self> {
                ::planus::UnionOffset::new(7, value.prepare(builder).downcast())
            }
        }

        impl ::planus::WriteAsUnion<Message> for Message {
            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::UnionOffset<Self> {
                match self {
                    Self::Connection(value) => Self::create_connection(builder, value),
                    Self::Quit(value) => Self::create_quit(builder, value),
                    Self::Speed(value) => Self::create_speed(builder, value),
                    Self::Paused(value) => Self::create_paused(builder, value),
                    Self::GameState(value) => Self::create_game_state(builder, value),
                    Self::AddRender(value) => Self::create_add_render(builder, value),
                    Self::RemoveRender(value) => Self::create_remove_render(builder, value),
                }
            }
        }

        impl ::planus::WriteAsOptionalUnion<Message> for Message {
            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::core::option::Option<::planus::UnionOffset<Self>> {
                ::core::option::Option::Some(::planus::WriteAsUnion::prepare(self, builder))
            }
        }

        /// Builder for serializing an instance of the [Message] type.
        ///
        /// Can be created using the [Message::builder] method.
        #[derive(Debug)]
        #[must_use]
        pub struct MessageBuilder<T>(T);

        impl MessageBuilder<::planus::Uninitialized> {
            /// Creates an instance of the [`Connection` variant](Message#variant.Connection).
            #[inline]
            pub fn connection<T>(self, value: T) -> MessageBuilder<::planus::Initialized<1, T>>
            where
                T: ::planus::WriteAsOffset<self::Connection>,
            {
                MessageBuilder(::planus::Initialized(value))
            }

            /// Creates an instance of the [`Quit` variant](Message#variant.Quit).
            #[inline]
            pub fn quit<T>(self, value: T) -> MessageBuilder<::planus::Initialized<2, T>>
            where
                T: ::planus::WriteAsOffset<self::Quit>,
            {
                MessageBuilder(::planus::Initialized(value))
            }

            /// Creates an instance of the [`Speed` variant](Message#variant.Speed).
            #[inline]
            pub fn speed<T>(self, value: T) -> MessageBuilder<::planus::Initialized<3, T>>
            where
                T: ::planus::WriteAsOffset<self::Speed>,
            {
                MessageBuilder(::planus::Initialized(value))
            }

            /// Creates an instance of the [`Paused` variant](Message#variant.Paused).
            #[inline]
            pub fn paused<T>(self, value: T) -> MessageBuilder<::planus::Initialized<4, T>>
            where
                T: ::planus::WriteAsOffset<self::Paused>,
            {
                MessageBuilder(::planus::Initialized(value))
            }

            /// Creates an instance of the [`GameState` variant](Message#variant.GameState).
            #[inline]
            pub fn game_state<T>(self, value: T) -> MessageBuilder<::planus::Initialized<5, T>>
            where
                T: ::planus::WriteAsOffset<self::GameState>,
            {
                MessageBuilder(::planus::Initialized(value))
            }

            /// Creates an instance of the [`AddRender` variant](Message#variant.AddRender).
            #[inline]
            pub fn add_render<T>(self, value: T) -> MessageBuilder<::planus::Initialized<6, T>>
            where
                T: ::planus::WriteAsOffset<self::AddRender>,
            {
                MessageBuilder(::planus::Initialized(value))
            }

            /// Creates an instance of the [`RemoveRender` variant](Message#variant.RemoveRender).
            #[inline]
            pub fn remove_render<T>(self, value: T) -> MessageBuilder<::planus::Initialized<7, T>>
            where
                T: ::planus::WriteAsOffset<self::RemoveRender>,
            {
                MessageBuilder(::planus::Initialized(value))
            }
        }

        impl<const N: u8, T> MessageBuilder<::planus::Initialized<N, T>> {
            /// Finish writing the builder to get an [UnionOffset](::planus::UnionOffset) to a serialized [Message].
            #[inline]
            pub fn finish(self, builder: &mut ::planus::Builder) -> ::planus::UnionOffset<Message>
            where
                Self: ::planus::WriteAsUnion<Message>,
            {
                ::planus::WriteAsUnion::prepare(&self, builder)
            }
        }

        impl<T> ::planus::WriteAsUnion<Message> for MessageBuilder<::planus::Initialized<1, T>>
        where
            T: ::planus::WriteAsOffset<self::Connection>,
        {
            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::UnionOffset<Message> {
                ::planus::UnionOffset::new(1, (self.0).0.prepare(builder).downcast())
            }
        }

        impl<T> ::planus::WriteAsOptionalUnion<Message> for MessageBuilder<::planus::Initialized<1, T>>
        where
            T: ::planus::WriteAsOffset<self::Connection>,
        {
            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::core::option::Option<::planus::UnionOffset<Message>> {
                ::core::option::Option::Some(::planus::WriteAsUnion::prepare(self, builder))
            }
        }
        impl<T> ::planus::WriteAsUnion<Message> for MessageBuilder<::planus::Initialized<2, T>>
        where
            T: ::planus::WriteAsOffset<self::Quit>,
        {
            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::UnionOffset<Message> {
                ::planus::UnionOffset::new(2, (self.0).0.prepare(builder).downcast())
            }
        }

        impl<T> ::planus::WriteAsOptionalUnion<Message> for MessageBuilder<::planus::Initialized<2, T>>
        where
            T: ::planus::WriteAsOffset<self::Quit>,
        {
            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::core::option::Option<::planus::UnionOffset<Message>> {
                ::core::option::Option::Some(::planus::WriteAsUnion::prepare(self, builder))
            }
        }
        impl<T> ::planus::WriteAsUnion<Message> for MessageBuilder<::planus::Initialized<3, T>>
        where
            T: ::planus::WriteAsOffset<self::Speed>,
        {
            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::UnionOffset<Message> {
                ::planus::UnionOffset::new(3, (self.0).0.prepare(builder).downcast())
            }
        }

        impl<T> ::planus::WriteAsOptionalUnion<Message> for MessageBuilder<::planus::Initialized<3, T>>
        where
            T: ::planus::WriteAsOffset<self::Speed>,
        {
            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::core::option::Option<::planus::UnionOffset<Message>> {
                ::core::option::Option::Some(::planus::WriteAsUnion::prepare(self, builder))
            }
        }
        impl<T> ::planus::WriteAsUnion<Message> for MessageBuilder<::planus::Initialized<4, T>>
        where
            T: ::planus::WriteAsOffset<self::Paused>,
        {
            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::UnionOffset<Message> {
                ::planus::UnionOffset::new(4, (self.0).0.prepare(builder).downcast())
            }
        }

        impl<T> ::planus::WriteAsOptionalUnion<Message> for MessageBuilder<::planus::Initialized<4, T>>
        where
            T: ::planus::WriteAsOffset<self::Paused>,
        {
            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::core::option::Option<::planus::UnionOffset<Message>> {
                ::core::option::Option::Some(::planus::WriteAsUnion::prepare(self, builder))
            }
        }
        impl<T> ::planus::WriteAsUnion<Message> for MessageBuilder<::planus::Initialized<5, T>>
        where
            T: ::planus::WriteAsOffset<self::GameState>,
        {
            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::UnionOffset<Message> {
                ::planus::UnionOffset::new(5, (self.0).0.prepare(builder).downcast())
            }
        }

        impl<T> ::planus::WriteAsOptionalUnion<Message> for MessageBuilder<::planus::Initialized<5, T>>
        where
            T: ::planus::WriteAsOffset<self::GameState>,
        {
            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::core::option::Option<::planus::UnionOffset<Message>> {
                ::core::option::Option::Some(::planus::WriteAsUnion::prepare(self, builder))
            }
        }
        impl<T> ::planus::WriteAsUnion<Message> for MessageBuilder<::planus::Initialized<6, T>>
        where
            T: ::planus::WriteAsOffset<self::AddRender>,
        {
            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::UnionOffset<Message> {
                ::planus::UnionOffset::new(6, (self.0).0.prepare(builder).downcast())
            }
        }

        impl<T> ::planus::WriteAsOptionalUnion<Message> for MessageBuilder<::planus::Initialized<6, T>>
        where
            T: ::planus::WriteAsOffset<self::AddRender>,
        {
            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::core::option::Option<::planus::UnionOffset<Message>> {
                ::core::option::Option::Some(::planus::WriteAsUnion::prepare(self, builder))
            }
        }
        impl<T> ::planus::WriteAsUnion<Message> for MessageBuilder<::planus::Initialized<7, T>>
        where
            T: ::planus::WriteAsOffset<self::RemoveRender>,
        {
            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::UnionOffset<Message> {
                ::planus::UnionOffset::new(7, (self.0).0.prepare(builder).downcast())
            }
        }

        impl<T> ::planus::WriteAsOptionalUnion<Message> for MessageBuilder<::planus::Initialized<7, T>>
        where
            T: ::planus::WriteAsOffset<self::RemoveRender>,
        {
            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::core::option::Option<::planus::UnionOffset<Message>> {
                ::core::option::Option::Some(::planus::WriteAsUnion::prepare(self, builder))
            }
        }

        /// Reference to a deserialized [Message].
        #[derive(Copy, Clone, Debug)]
        pub enum MessageRef<'a> {
            Connection(self::ConnectionRef<'a>),
            Quit(self::QuitRef<'a>),
            Speed(self::SpeedRef<'a>),
            Paused(self::PausedRef<'a>),
            GameState(self::GameStateRef<'a>),
            AddRender(self::AddRenderRef<'a>),
            RemoveRender(self::RemoveRenderRef<'a>),
        }

        impl<'a> ::core::convert::TryFrom<MessageRef<'a>> for Message {
            type Error = ::planus::Error;

            fn try_from(value: MessageRef<'a>) -> ::planus::Result<Self> {
                ::core::result::Result::Ok(match value {
                    MessageRef::Connection(value) => {
                        Self::Connection(::planus::alloc::boxed::Box::new(::core::convert::TryFrom::try_from(value)?))
                    }

                    MessageRef::Quit(value) => {
                        Self::Quit(::planus::alloc::boxed::Box::new(::core::convert::TryFrom::try_from(value)?))
                    }

                    MessageRef::Speed(value) => {
                        Self::Speed(::planus::alloc::boxed::Box::new(::core::convert::TryFrom::try_from(value)?))
                    }

                    MessageRef::Paused(value) => {
                        Self::Paused(::planus::alloc::boxed::Box::new(::core::convert::TryFrom::try_from(value)?))
                    }

                    MessageRef::GameState(value) => {
                        Self::GameState(::planus::alloc::boxed::Box::new(::core::convert::TryFrom::try_from(value)?))
                    }

                    MessageRef::AddRender(value) => {
                        Self::AddRender(::planus::alloc::boxed::Box::new(::core::convert::TryFrom::try_from(value)?))
                    }

                    MessageRef::RemoveRender(value) => {
                        Self::RemoveRender(::planus::alloc::boxed::Box::new(::core::convert::TryFrom::try_from(value)?))
                    }
                })
            }
        }

        impl<'a> ::planus::TableReadUnion<'a> for MessageRef<'a> {
            fn from_buffer(
                buffer: ::planus::SliceWithStartOffset<'a>,
                tag: u8,
                field_offset: usize,
            ) -> ::core::result::Result<Self, ::planus::errors::ErrorKind> {
                match tag {
                    1 => {
                        ::core::result::Result::Ok(Self::Connection(::planus::TableRead::from_buffer(buffer, field_offset)?))
                    }
                    2 => ::core::result::Result::Ok(Self::Quit(::planus::TableRead::from_buffer(buffer, field_offset)?)),
                    3 => ::core::result::Result::Ok(Self::Speed(::planus::TableRead::from_buffer(buffer, field_offset)?)),
                    4 => ::core::result::Result::Ok(Self::Paused(::planus::TableRead::from_buffer(buffer, field_offset)?)),
                    5 => {
                        ::core::result::Result::Ok(Self::GameState(::planus::TableRead::from_buffer(buffer, field_offset)?))
                    }
                    6 => {
                        ::core::result::Result::Ok(Self::AddRender(::planus::TableRead::from_buffer(buffer, field_offset)?))
                    }
                    7 => ::core::result::Result::Ok(Self::RemoveRender(::planus::TableRead::from_buffer(
                        buffer,
                        field_offset,
                    )?)),
                    _ => ::core::result::Result::Err(::planus::errors::ErrorKind::UnknownUnionTag { tag }),
                }
            }
        }

        impl<'a> ::planus::VectorReadUnion<'a> for MessageRef<'a> {
            const VECTOR_NAME: &'static str = "[MessageRef]";
        }

        /// The table `Packet` in the namespace `rocketsim`
        ///
        /// Generated from these locations:
        /// * Table `Packet` in the file `spec/core.fbs:28`
        #[derive(Clone, Debug, PartialEq, PartialOrd, ::serde::Serialize, ::serde::Deserialize)]
        pub struct Packet {
            /// The field `message` in the table `Packet`
            pub message: self::Message,
        }

        impl Packet {
            /// Creates a [PacketBuilder] for serializing an instance of this table.
            #[inline]
            pub fn builder() -> PacketBuilder<()> {
                PacketBuilder(())
            }

            #[allow(clippy::too_many_arguments)]
            pub fn create(
                builder: &mut ::planus::Builder,
                field_message: impl ::planus::WriteAsUnion<self::Message>,
            ) -> ::planus::Offset<Self> {
                let prepared_message = field_message.prepare(builder);

                let mut table_writer: ::planus::table_writer::TableWriter<8> = ::core::default::Default::default();
                table_writer.write_entry::<::planus::Offset<self::Message>>(1);
                table_writer.write_entry::<u8>(0);

                unsafe {
                    table_writer.finish(builder, |object_writer| {
                        object_writer.write::<_, _, 4>(&prepared_message.offset());
                        object_writer.write::<_, _, 1>(&prepared_message.tag());
                    });
                }
                builder.current_offset()
            }
        }

        impl ::planus::WriteAs<::planus::Offset<Packet>> for Packet {
            type Prepared = ::planus::Offset<Self>;

            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<Packet> {
                ::planus::WriteAsOffset::prepare(self, builder)
            }
        }

        impl ::planus::WriteAsOptional<::planus::Offset<Packet>> for Packet {
            type Prepared = ::planus::Offset<Self>;

            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::core::option::Option<::planus::Offset<Packet>> {
                ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
            }
        }

        impl ::planus::WriteAsOffset<Packet> for Packet {
            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<Packet> {
                Packet::create(builder, &self.message)
            }
        }

        /// Builder for serializing an instance of the [Packet] type.
        ///
        /// Can be created using the [Packet::builder] method.
        #[derive(Debug)]
        #[must_use]
        pub struct PacketBuilder<State>(State);

        impl PacketBuilder<()> {
            /// Setter for the [`message` field](Packet#structfield.message).
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn message<T0>(self, value: T0) -> PacketBuilder<(T0,)>
            where
                T0: ::planus::WriteAsUnion<self::Message>,
            {
                PacketBuilder((value,))
            }
        }

        impl<T0> PacketBuilder<(T0,)> {
            /// Finish writing the builder to get an [Offset](::planus::Offset) to a serialized [Packet].
            #[inline]
            pub fn finish(self, builder: &mut ::planus::Builder) -> ::planus::Offset<Packet>
            where
                Self: ::planus::WriteAsOffset<Packet>,
            {
                ::planus::WriteAsOffset::prepare(&self, builder)
            }
        }

        impl<T0: ::planus::WriteAsUnion<self::Message>> ::planus::WriteAs<::planus::Offset<Packet>> for PacketBuilder<(T0,)> {
            type Prepared = ::planus::Offset<Packet>;

            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<Packet> {
                ::planus::WriteAsOffset::prepare(self, builder)
            }
        }

        impl<T0: ::planus::WriteAsUnion<self::Message>> ::planus::WriteAsOptional<::planus::Offset<Packet>>
            for PacketBuilder<(T0,)>
        {
            type Prepared = ::planus::Offset<Packet>;

            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::core::option::Option<::planus::Offset<Packet>> {
                ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
            }
        }

        impl<T0: ::planus::WriteAsUnion<self::Message>> ::planus::WriteAsOffset<Packet> for PacketBuilder<(T0,)> {
            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<Packet> {
                let (v0,) = &self.0;
                Packet::create(builder, v0)
            }
        }

        /// Reference to a deserialized [Packet].
        #[derive(Copy, Clone)]
        pub struct PacketRef<'a>(#[allow(dead_code)] ::planus::table_reader::Table<'a>);

        impl<'a> PacketRef<'a> {
            /// Getter for the [`message` field](Packet#structfield.message).
            #[inline]
            pub fn message(&self) -> ::planus::Result<self::MessageRef<'a>> {
                self.0.access_union_required(0, "Packet", "message")
            }
        }

        impl<'a> ::core::fmt::Debug for PacketRef<'a> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                let mut f = f.debug_struct("PacketRef");
                f.field("message", &self.message());
                f.finish()
            }
        }

        impl<'a> ::core::convert::TryFrom<PacketRef<'a>> for Packet {
            type Error = ::planus::Error;

            #[allow(unreachable_code)]
            fn try_from(value: PacketRef<'a>) -> ::planus::Result<Self> {
                ::core::result::Result::Ok(Self {
                    message: ::core::convert::TryInto::try_into(value.message()?)?,
                })
            }
        }

        impl<'a> ::planus::TableRead<'a> for PacketRef<'a> {
            #[inline]
            fn from_buffer(
                buffer: ::planus::SliceWithStartOffset<'a>,
                offset: usize,
            ) -> ::core::result::Result<Self, ::planus::errors::ErrorKind> {
                ::core::result::Result::Ok(Self(::planus::table_reader::Table::from_buffer(buffer, offset)?))
            }
        }

        impl<'a> ::planus::VectorReadInner<'a> for PacketRef<'a> {
            type Error = ::planus::Error;
            const STRIDE: usize = 4;

            unsafe fn from_buffer(buffer: ::planus::SliceWithStartOffset<'a>, offset: usize) -> ::planus::Result<Self> {
                ::planus::TableRead::from_buffer(buffer, offset)
                    .map_err(|error_kind| error_kind.with_error_location("[PacketRef]", "get", buffer.offset_from_start))
            }
        }

        /// # Safety
        /// The planus compiler generates implementations that initialize
        /// the bytes in `write_values`.
        unsafe impl ::planus::VectorWrite<::planus::Offset<Packet>> for Packet {
            type Value = ::planus::Offset<Packet>;
            const STRIDE: usize = 4;
            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> Self::Value {
                ::planus::WriteAs::prepare(self, builder)
            }

            #[inline]
            unsafe fn write_values(
                values: &[::planus::Offset<Packet>],
                bytes: *mut ::core::mem::MaybeUninit<u8>,
                buffer_position: u32,
            ) {
                let bytes = bytes as *mut [::core::mem::MaybeUninit<u8>; 4];
                for (i, v) in ::core::iter::Iterator::enumerate(values.iter()) {
                    ::planus::WriteAsPrimitive::write(
                        v,
                        ::planus::Cursor::new(unsafe { &mut *bytes.add(i) }),
                        buffer_position - (Self::STRIDE * i) as u32,
                    );
                }
            }
        }

        impl<'a> ::planus::ReadAsRoot<'a> for PacketRef<'a> {
            fn read_as_root(slice: &'a [u8]) -> ::planus::Result<Self> {
                ::planus::TableRead::from_buffer(
                    ::planus::SliceWithStartOffset {
                        buffer: slice,
                        offset_from_start: 0,
                    },
                    0,
                )
                .map_err(|error_kind| error_kind.with_error_location("[PacketRef]", "read_as_root", 0))
            }
        }

        /// The enum `GameMode` in the namespace `rocketsim`
        ///
        /// Generated from these locations:
        /// * Enum `GameMode` in the file `spec/game_state.fbs:5`
        #[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, ::serde::Serialize, ::serde::Deserialize)]
        #[repr(u8)]
        pub enum GameMode {
            /// The variant `Soccar` in the enum `GameMode`
            Soccar = 0,

            /// The variant `Hoops` in the enum `GameMode`
            Hoops = 1,

            /// The variant `Heatseeker` in the enum `GameMode`
            Heatseeker = 2,

            /// The variant `Snowday` in the enum `GameMode`
            Snowday = 3,

            /// The variant `Dropshot` in the enum `GameMode`
            Dropshot = 4,

            /// The variant `TheVoid` in the enum `GameMode`
            TheVoid = 5,
        }

        impl GameMode {
            /// Array containing all valid variants of GameMode
            pub const ENUM_VALUES: [Self; 6] = [
                Self::Soccar,
                Self::Hoops,
                Self::Heatseeker,
                Self::Snowday,
                Self::Dropshot,
                Self::TheVoid,
            ];
        }

        impl ::core::convert::TryFrom<u8> for GameMode {
            type Error = ::planus::errors::UnknownEnumTagKind;
            #[inline]
            fn try_from(value: u8) -> ::core::result::Result<Self, ::planus::errors::UnknownEnumTagKind> {
                #[allow(clippy::match_single_binding)]
                match value {
                    0 => ::core::result::Result::Ok(GameMode::Soccar),
                    1 => ::core::result::Result::Ok(GameMode::Hoops),
                    2 => ::core::result::Result::Ok(GameMode::Heatseeker),
                    3 => ::core::result::Result::Ok(GameMode::Snowday),
                    4 => ::core::result::Result::Ok(GameMode::Dropshot),
                    5 => ::core::result::Result::Ok(GameMode::TheVoid),

                    _ => ::core::result::Result::Err(::planus::errors::UnknownEnumTagKind { tag: value as i128 }),
                }
            }
        }

        impl ::core::convert::From<GameMode> for u8 {
            #[inline]
            fn from(value: GameMode) -> Self {
                value as u8
            }
        }

        /// # Safety
        /// The Planus compiler correctly calculates `ALIGNMENT` and `SIZE`.
        unsafe impl ::planus::Primitive for GameMode {
            const ALIGNMENT: usize = 1;
            const SIZE: usize = 1;
        }

        impl ::planus::WriteAsPrimitive<GameMode> for GameMode {
            #[inline]
            fn write<const N: usize>(&self, cursor: ::planus::Cursor<'_, N>, buffer_position: u32) {
                (*self as u8).write(cursor, buffer_position);
            }
        }

        impl ::planus::WriteAs<GameMode> for GameMode {
            type Prepared = Self;

            #[inline]
            fn prepare(&self, _builder: &mut ::planus::Builder) -> GameMode {
                *self
            }
        }

        impl ::planus::WriteAsDefault<GameMode, GameMode> for GameMode {
            type Prepared = Self;

            #[inline]
            fn prepare(&self, _builder: &mut ::planus::Builder, default: &GameMode) -> ::core::option::Option<GameMode> {
                if self == default {
                    ::core::option::Option::None
                } else {
                    ::core::option::Option::Some(*self)
                }
            }
        }

        impl ::planus::WriteAsOptional<GameMode> for GameMode {
            type Prepared = Self;

            #[inline]
            fn prepare(&self, _builder: &mut ::planus::Builder) -> ::core::option::Option<GameMode> {
                ::core::option::Option::Some(*self)
            }
        }

        impl<'buf> ::planus::TableRead<'buf> for GameMode {
            #[inline]
            fn from_buffer(
                buffer: ::planus::SliceWithStartOffset<'buf>,
                offset: usize,
            ) -> ::core::result::Result<Self, ::planus::errors::ErrorKind> {
                let n: u8 = ::planus::TableRead::from_buffer(buffer, offset)?;
                ::core::result::Result::Ok(::core::convert::TryInto::try_into(n)?)
            }
        }

        impl<'buf> ::planus::VectorReadInner<'buf> for GameMode {
            type Error = ::planus::errors::UnknownEnumTag;
            const STRIDE: usize = 1;
            #[inline]
            unsafe fn from_buffer(
                buffer: ::planus::SliceWithStartOffset<'buf>,
                offset: usize,
            ) -> ::core::result::Result<Self, ::planus::errors::UnknownEnumTag> {
                let value = unsafe { *buffer.buffer.get_unchecked(offset) };
                let value: ::core::result::Result<Self, _> = ::core::convert::TryInto::try_into(value);
                value.map_err(|error_kind| {
                    error_kind.with_error_location("GameMode", "VectorRead::from_buffer", buffer.offset_from_start)
                })
            }
        }

        /// # Safety
        /// The planus compiler generates implementations that initialize
        /// the bytes in `write_values`.
        unsafe impl ::planus::VectorWrite<GameMode> for GameMode {
            const STRIDE: usize = 1;

            type Value = Self;

            #[inline]
            fn prepare(&self, _builder: &mut ::planus::Builder) -> Self {
                *self
            }

            #[inline]
            unsafe fn write_values(values: &[Self], bytes: *mut ::core::mem::MaybeUninit<u8>, buffer_position: u32) {
                let bytes = bytes as *mut [::core::mem::MaybeUninit<u8>; 1];
                for (i, v) in ::core::iter::Iterator::enumerate(values.iter()) {
                    ::planus::WriteAsPrimitive::write(
                        v,
                        ::planus::Cursor::new(unsafe { &mut *bytes.add(i) }),
                        buffer_position - i as u32,
                    );
                }
            }
        }

        /// The enum `Team` in the namespace `rocketsim`
        ///
        /// Generated from these locations:
        /// * Enum `Team` in the file `spec/game_state.fbs:14`
        #[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, ::serde::Serialize, ::serde::Deserialize)]
        #[repr(u8)]
        pub enum Team {
            /// The variant `Blue` in the enum `Team`
            Blue = 0,

            /// The variant `Orange` in the enum `Team`
            Orange = 1,
        }

        impl Team {
            /// Array containing all valid variants of Team
            pub const ENUM_VALUES: [Self; 2] = [Self::Blue, Self::Orange];
        }

        impl ::core::convert::TryFrom<u8> for Team {
            type Error = ::planus::errors::UnknownEnumTagKind;
            #[inline]
            fn try_from(value: u8) -> ::core::result::Result<Self, ::planus::errors::UnknownEnumTagKind> {
                #[allow(clippy::match_single_binding)]
                match value {
                    0 => ::core::result::Result::Ok(Team::Blue),
                    1 => ::core::result::Result::Ok(Team::Orange),

                    _ => ::core::result::Result::Err(::planus::errors::UnknownEnumTagKind { tag: value as i128 }),
                }
            }
        }

        impl ::core::convert::From<Team> for u8 {
            #[inline]
            fn from(value: Team) -> Self {
                value as u8
            }
        }

        /// # Safety
        /// The Planus compiler correctly calculates `ALIGNMENT` and `SIZE`.
        unsafe impl ::planus::Primitive for Team {
            const ALIGNMENT: usize = 1;
            const SIZE: usize = 1;
        }

        impl ::planus::WriteAsPrimitive<Team> for Team {
            #[inline]
            fn write<const N: usize>(&self, cursor: ::planus::Cursor<'_, N>, buffer_position: u32) {
                (*self as u8).write(cursor, buffer_position);
            }
        }

        impl ::planus::WriteAs<Team> for Team {
            type Prepared = Self;

            #[inline]
            fn prepare(&self, _builder: &mut ::planus::Builder) -> Team {
                *self
            }
        }

        impl ::planus::WriteAsDefault<Team, Team> for Team {
            type Prepared = Self;

            #[inline]
            fn prepare(&self, _builder: &mut ::planus::Builder, default: &Team) -> ::core::option::Option<Team> {
                if self == default {
                    ::core::option::Option::None
                } else {
                    ::core::option::Option::Some(*self)
                }
            }
        }

        impl ::planus::WriteAsOptional<Team> for Team {
            type Prepared = Self;

            #[inline]
            fn prepare(&self, _builder: &mut ::planus::Builder) -> ::core::option::Option<Team> {
                ::core::option::Option::Some(*self)
            }
        }

        impl<'buf> ::planus::TableRead<'buf> for Team {
            #[inline]
            fn from_buffer(
                buffer: ::planus::SliceWithStartOffset<'buf>,
                offset: usize,
            ) -> ::core::result::Result<Self, ::planus::errors::ErrorKind> {
                let n: u8 = ::planus::TableRead::from_buffer(buffer, offset)?;
                ::core::result::Result::Ok(::core::convert::TryInto::try_into(n)?)
            }
        }

        impl<'buf> ::planus::VectorReadInner<'buf> for Team {
            type Error = ::planus::errors::UnknownEnumTag;
            const STRIDE: usize = 1;
            #[inline]
            unsafe fn from_buffer(
                buffer: ::planus::SliceWithStartOffset<'buf>,
                offset: usize,
            ) -> ::core::result::Result<Self, ::planus::errors::UnknownEnumTag> {
                let value = unsafe { *buffer.buffer.get_unchecked(offset) };
                let value: ::core::result::Result<Self, _> = ::core::convert::TryInto::try_into(value);
                value.map_err(|error_kind| {
                    error_kind.with_error_location("Team", "VectorRead::from_buffer", buffer.offset_from_start)
                })
            }
        }

        /// # Safety
        /// The planus compiler generates implementations that initialize
        /// the bytes in `write_values`.
        unsafe impl ::planus::VectorWrite<Team> for Team {
            const STRIDE: usize = 1;

            type Value = Self;

            #[inline]
            fn prepare(&self, _builder: &mut ::planus::Builder) -> Self {
                *self
            }

            #[inline]
            unsafe fn write_values(values: &[Self], bytes: *mut ::core::mem::MaybeUninit<u8>, buffer_position: u32) {
                let bytes = bytes as *mut [::core::mem::MaybeUninit<u8>; 1];
                for (i, v) in ::core::iter::Iterator::enumerate(values.iter()) {
                    ::planus::WriteAsPrimitive::write(
                        v,
                        ::planus::Cursor::new(unsafe { &mut *bytes.add(i) }),
                        buffer_position - i as u32,
                    );
                }
            }
        }

        /// The enum `TileState` in the namespace `rocketsim`
        ///
        /// Generated from these locations:
        /// * Enum `TileState` in the file `spec/game_state.fbs:19`
        #[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, ::serde::Serialize, ::serde::Deserialize)]
        #[repr(u8)]
        pub enum TileState {
            /// The variant `Full` in the enum `TileState`
            Full = 0,

            /// The variant `Damaged` in the enum `TileState`
            Damaged = 1,

            /// The variant `Broken` in the enum `TileState`
            Broken = 2,
        }

        impl TileState {
            /// Array containing all valid variants of TileState
            pub const ENUM_VALUES: [Self; 3] = [Self::Full, Self::Damaged, Self::Broken];
        }

        impl ::core::convert::TryFrom<u8> for TileState {
            type Error = ::planus::errors::UnknownEnumTagKind;
            #[inline]
            fn try_from(value: u8) -> ::core::result::Result<Self, ::planus::errors::UnknownEnumTagKind> {
                #[allow(clippy::match_single_binding)]
                match value {
                    0 => ::core::result::Result::Ok(TileState::Full),
                    1 => ::core::result::Result::Ok(TileState::Damaged),
                    2 => ::core::result::Result::Ok(TileState::Broken),

                    _ => ::core::result::Result::Err(::planus::errors::UnknownEnumTagKind { tag: value as i128 }),
                }
            }
        }

        impl ::core::convert::From<TileState> for u8 {
            #[inline]
            fn from(value: TileState) -> Self {
                value as u8
            }
        }

        /// # Safety
        /// The Planus compiler correctly calculates `ALIGNMENT` and `SIZE`.
        unsafe impl ::planus::Primitive for TileState {
            const ALIGNMENT: usize = 1;
            const SIZE: usize = 1;
        }

        impl ::planus::WriteAsPrimitive<TileState> for TileState {
            #[inline]
            fn write<const N: usize>(&self, cursor: ::planus::Cursor<'_, N>, buffer_position: u32) {
                (*self as u8).write(cursor, buffer_position);
            }
        }

        impl ::planus::WriteAs<TileState> for TileState {
            type Prepared = Self;

            #[inline]
            fn prepare(&self, _builder: &mut ::planus::Builder) -> TileState {
                *self
            }
        }

        impl ::planus::WriteAsDefault<TileState, TileState> for TileState {
            type Prepared = Self;

            #[inline]
            fn prepare(&self, _builder: &mut ::planus::Builder, default: &TileState) -> ::core::option::Option<TileState> {
                if self == default {
                    ::core::option::Option::None
                } else {
                    ::core::option::Option::Some(*self)
                }
            }
        }

        impl ::planus::WriteAsOptional<TileState> for TileState {
            type Prepared = Self;

            #[inline]
            fn prepare(&self, _builder: &mut ::planus::Builder) -> ::core::option::Option<TileState> {
                ::core::option::Option::Some(*self)
            }
        }

        impl<'buf> ::planus::TableRead<'buf> for TileState {
            #[inline]
            fn from_buffer(
                buffer: ::planus::SliceWithStartOffset<'buf>,
                offset: usize,
            ) -> ::core::result::Result<Self, ::planus::errors::ErrorKind> {
                let n: u8 = ::planus::TableRead::from_buffer(buffer, offset)?;
                ::core::result::Result::Ok(::core::convert::TryInto::try_into(n)?)
            }
        }

        impl<'buf> ::planus::VectorReadInner<'buf> for TileState {
            type Error = ::planus::errors::UnknownEnumTag;
            const STRIDE: usize = 1;
            #[inline]
            unsafe fn from_buffer(
                buffer: ::planus::SliceWithStartOffset<'buf>,
                offset: usize,
            ) -> ::core::result::Result<Self, ::planus::errors::UnknownEnumTag> {
                let value = unsafe { *buffer.buffer.get_unchecked(offset) };
                let value: ::core::result::Result<Self, _> = ::core::convert::TryInto::try_into(value);
                value.map_err(|error_kind| {
                    error_kind.with_error_location("TileState", "VectorRead::from_buffer", buffer.offset_from_start)
                })
            }
        }

        /// # Safety
        /// The planus compiler generates implementations that initialize
        /// the bytes in `write_values`.
        unsafe impl ::planus::VectorWrite<TileState> for TileState {
            const STRIDE: usize = 1;

            type Value = Self;

            #[inline]
            fn prepare(&self, _builder: &mut ::planus::Builder) -> Self {
                *self
            }

            #[inline]
            unsafe fn write_values(values: &[Self], bytes: *mut ::core::mem::MaybeUninit<u8>, buffer_position: u32) {
                let bytes = bytes as *mut [::core::mem::MaybeUninit<u8>; 1];
                for (i, v) in ::core::iter::Iterator::enumerate(values.iter()) {
                    ::planus::WriteAsPrimitive::write(
                        v,
                        ::planus::Cursor::new(unsafe { &mut *bytes.add(i) }),
                        buffer_position - i as u32,
                    );
                }
            }
        }

        /// The struct `Mat3` in the namespace `rocketsim`
        ///
        /// Generated from these locations:
        /// * Struct `Mat3` in the file `spec/game_state.fbs:25`
        #[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Default, ::serde::Serialize, ::serde::Deserialize)]
        pub struct Mat3 {
            /// The field `forward` in the struct `Mat3`
            pub forward: self::Vec3,

            /// The field `right` in the struct `Mat3`
            pub right: self::Vec3,

            /// The field `up` in the struct `Mat3`
            pub up: self::Vec3,
        }

        /// # Safety
        /// The Planus compiler correctly calculates `ALIGNMENT` and `SIZE`.
        unsafe impl ::planus::Primitive for Mat3 {
            const ALIGNMENT: usize = 4;
            const SIZE: usize = 36;
        }

        #[allow(clippy::identity_op)]
        impl ::planus::WriteAsPrimitive<Mat3> for Mat3 {
            #[inline]
            fn write<const N: usize>(&self, cursor: ::planus::Cursor<'_, N>, buffer_position: u32) {
                let (cur, cursor) = cursor.split::<12, 24>();
                self.forward.write(cur, buffer_position - 0);
                let (cur, cursor) = cursor.split::<12, 12>();
                self.right.write(cur, buffer_position - 12);
                let (cur, cursor) = cursor.split::<12, 0>();
                self.up.write(cur, buffer_position - 24);
                cursor.finish([]);
            }
        }

        impl ::planus::WriteAsOffset<Mat3> for Mat3 {
            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<Mat3> {
                unsafe {
                    builder.write_with(36, 3, |buffer_position, bytes| {
                        let bytes = bytes.as_mut_ptr();

                        ::planus::WriteAsPrimitive::write(
                            self,
                            ::planus::Cursor::new(&mut *(bytes as *mut [::core::mem::MaybeUninit<u8>; 36])),
                            buffer_position,
                        );
                    });
                }
                builder.current_offset()
            }
        }

        impl ::planus::WriteAs<Mat3> for Mat3 {
            type Prepared = Self;
            #[inline]
            fn prepare(&self, _builder: &mut ::planus::Builder) -> Self {
                *self
            }
        }

        impl ::planus::WriteAsOptional<Mat3> for Mat3 {
            type Prepared = Self;
            #[inline]
            fn prepare(&self, _builder: &mut ::planus::Builder) -> ::core::option::Option<Self> {
                ::core::option::Option::Some(*self)
            }
        }

        /// Reference to a deserialized [Mat3].
        #[derive(Copy, Clone)]
        pub struct Mat3Ref<'a>(::planus::ArrayWithStartOffset<'a, 36>);

        impl<'a> Mat3Ref<'a> {
            /// Getter for the [`forward` field](Mat3#structfield.forward).
            pub fn forward(&self) -> self::Vec3Ref<'a> {
                let buffer = self.0.advance_as_array::<12>(0).unwrap();

                ::core::convert::From::from(buffer)
            }

            /// Getter for the [`right` field](Mat3#structfield.right).
            pub fn right(&self) -> self::Vec3Ref<'a> {
                let buffer = self.0.advance_as_array::<12>(12).unwrap();

                ::core::convert::From::from(buffer)
            }

            /// Getter for the [`up` field](Mat3#structfield.up).
            pub fn up(&self) -> self::Vec3Ref<'a> {
                let buffer = self.0.advance_as_array::<12>(24).unwrap();

                ::core::convert::From::from(buffer)
            }
        }

        impl<'a> ::core::fmt::Debug for Mat3Ref<'a> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                let mut f = f.debug_struct("Mat3Ref");
                f.field("forward", &self.forward());
                f.field("right", &self.right());
                f.field("up", &self.up());
                f.finish()
            }
        }

        impl<'a> ::core::convert::From<::planus::ArrayWithStartOffset<'a, 36>> for Mat3Ref<'a> {
            fn from(array: ::planus::ArrayWithStartOffset<'a, 36>) -> Self {
                Self(array)
            }
        }

        impl<'a> ::core::convert::From<Mat3Ref<'a>> for Mat3 {
            #[allow(unreachable_code)]
            fn from(value: Mat3Ref<'a>) -> Self {
                Self {
                    forward: ::core::convert::From::from(value.forward()),
                    right: ::core::convert::From::from(value.right()),
                    up: ::core::convert::From::from(value.up()),
                }
            }
        }

        impl<'a, 'b> ::core::cmp::PartialEq<Mat3Ref<'a>> for Mat3Ref<'b> {
            fn eq(&self, other: &Mat3Ref<'_>) -> bool {
                self.forward() == other.forward() && self.right() == other.right() && self.up() == other.up()
            }
        }

        impl<'a, 'b> ::core::cmp::PartialOrd<Mat3Ref<'a>> for Mat3Ref<'b> {
            fn partial_cmp(&self, other: &Mat3Ref<'_>) -> ::core::option::Option<::core::cmp::Ordering> {
                match self.forward().partial_cmp(&other.forward()) {
                    ::core::option::Option::Some(::core::cmp::Ordering::Equal) => (),
                    o => return o,
                }

                match self.right().partial_cmp(&other.right()) {
                    ::core::option::Option::Some(::core::cmp::Ordering::Equal) => (),
                    o => return o,
                }

                self.up().partial_cmp(&other.up())
            }
        }

        impl<'a> ::planus::TableRead<'a> for Mat3Ref<'a> {
            #[inline]
            fn from_buffer(
                buffer: ::planus::SliceWithStartOffset<'a>,
                offset: usize,
            ) -> ::core::result::Result<Self, ::planus::errors::ErrorKind> {
                let buffer = buffer.advance_as_array::<36>(offset)?;
                ::core::result::Result::Ok(Self(buffer))
            }
        }

        impl<'a> ::planus::VectorRead<'a> for Mat3Ref<'a> {
            const STRIDE: usize = 36;

            #[inline]
            unsafe fn from_buffer(buffer: ::planus::SliceWithStartOffset<'a>, offset: usize) -> Self {
                Self(unsafe { buffer.unchecked_advance_as_array(offset) })
            }
        }

        /// # Safety
        /// The planus compiler generates implementations that initialize
        /// the bytes in `write_values`.
        unsafe impl ::planus::VectorWrite<Mat3> for Mat3 {
            const STRIDE: usize = 36;

            type Value = Mat3;

            #[inline]
            fn prepare(&self, _builder: &mut ::planus::Builder) -> Self::Value {
                *self
            }

            #[inline]
            unsafe fn write_values(values: &[Mat3], bytes: *mut ::core::mem::MaybeUninit<u8>, buffer_position: u32) {
                let bytes = bytes as *mut [::core::mem::MaybeUninit<u8>; 36];
                for (i, v) in ::core::iter::Iterator::enumerate(values.iter()) {
                    ::planus::WriteAsPrimitive::write(
                        v,
                        ::planus::Cursor::new(unsafe { &mut *bytes.add(i) }),
                        buffer_position - (36 * i) as u32,
                    );
                }
            }
        }

        ///  Physics snapshot shared by cars and ball.
        ///
        /// Generated from these locations:
        /// * Struct `PhysState` in the file `spec/game_state.fbs:32`
        #[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Default, ::serde::Serialize, ::serde::Deserialize)]
        pub struct PhysState {
            /// The field `pos` in the struct `PhysState`
            pub pos: self::Vec3,

            /// The field `rot_mat` in the struct `PhysState`
            pub rot_mat: self::Mat3,

            /// The field `vel` in the struct `PhysState`
            pub vel: self::Vec3,

            /// The field `ang_vel` in the struct `PhysState`
            pub ang_vel: self::Vec3,
        }

        /// # Safety
        /// The Planus compiler correctly calculates `ALIGNMENT` and `SIZE`.
        unsafe impl ::planus::Primitive for PhysState {
            const ALIGNMENT: usize = 4;
            const SIZE: usize = 72;
        }

        #[allow(clippy::identity_op)]
        impl ::planus::WriteAsPrimitive<PhysState> for PhysState {
            #[inline]
            fn write<const N: usize>(&self, cursor: ::planus::Cursor<'_, N>, buffer_position: u32) {
                let (cur, cursor) = cursor.split::<12, 60>();
                self.pos.write(cur, buffer_position - 0);
                let (cur, cursor) = cursor.split::<36, 24>();
                self.rot_mat.write(cur, buffer_position - 12);
                let (cur, cursor) = cursor.split::<12, 12>();
                self.vel.write(cur, buffer_position - 48);
                let (cur, cursor) = cursor.split::<12, 0>();
                self.ang_vel.write(cur, buffer_position - 60);
                cursor.finish([]);
            }
        }

        impl ::planus::WriteAsOffset<PhysState> for PhysState {
            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<PhysState> {
                unsafe {
                    builder.write_with(72, 3, |buffer_position, bytes| {
                        let bytes = bytes.as_mut_ptr();

                        ::planus::WriteAsPrimitive::write(
                            self,
                            ::planus::Cursor::new(&mut *(bytes as *mut [::core::mem::MaybeUninit<u8>; 72])),
                            buffer_position,
                        );
                    });
                }
                builder.current_offset()
            }
        }

        impl ::planus::WriteAs<PhysState> for PhysState {
            type Prepared = Self;
            #[inline]
            fn prepare(&self, _builder: &mut ::planus::Builder) -> Self {
                *self
            }
        }

        impl ::planus::WriteAsOptional<PhysState> for PhysState {
            type Prepared = Self;
            #[inline]
            fn prepare(&self, _builder: &mut ::planus::Builder) -> ::core::option::Option<Self> {
                ::core::option::Option::Some(*self)
            }
        }

        /// Reference to a deserialized [PhysState].
        #[derive(Copy, Clone)]
        pub struct PhysStateRef<'a>(::planus::ArrayWithStartOffset<'a, 72>);

        impl<'a> PhysStateRef<'a> {
            /// Getter for the [`pos` field](PhysState#structfield.pos).
            pub fn pos(&self) -> self::Vec3Ref<'a> {
                let buffer = self.0.advance_as_array::<12>(0).unwrap();

                ::core::convert::From::from(buffer)
            }

            /// Getter for the [`rot_mat` field](PhysState#structfield.rot_mat).
            pub fn rot_mat(&self) -> self::Mat3Ref<'a> {
                let buffer = self.0.advance_as_array::<36>(12).unwrap();

                ::core::convert::From::from(buffer)
            }

            /// Getter for the [`vel` field](PhysState#structfield.vel).
            pub fn vel(&self) -> self::Vec3Ref<'a> {
                let buffer = self.0.advance_as_array::<12>(48).unwrap();

                ::core::convert::From::from(buffer)
            }

            /// Getter for the [`ang_vel` field](PhysState#structfield.ang_vel).
            pub fn ang_vel(&self) -> self::Vec3Ref<'a> {
                let buffer = self.0.advance_as_array::<12>(60).unwrap();

                ::core::convert::From::from(buffer)
            }
        }

        impl<'a> ::core::fmt::Debug for PhysStateRef<'a> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                let mut f = f.debug_struct("PhysStateRef");
                f.field("pos", &self.pos());
                f.field("rot_mat", &self.rot_mat());
                f.field("vel", &self.vel());
                f.field("ang_vel", &self.ang_vel());
                f.finish()
            }
        }

        impl<'a> ::core::convert::From<::planus::ArrayWithStartOffset<'a, 72>> for PhysStateRef<'a> {
            fn from(array: ::planus::ArrayWithStartOffset<'a, 72>) -> Self {
                Self(array)
            }
        }

        impl<'a> ::core::convert::From<PhysStateRef<'a>> for PhysState {
            #[allow(unreachable_code)]
            fn from(value: PhysStateRef<'a>) -> Self {
                Self {
                    pos: ::core::convert::From::from(value.pos()),
                    rot_mat: ::core::convert::From::from(value.rot_mat()),
                    vel: ::core::convert::From::from(value.vel()),
                    ang_vel: ::core::convert::From::from(value.ang_vel()),
                }
            }
        }

        impl<'a, 'b> ::core::cmp::PartialEq<PhysStateRef<'a>> for PhysStateRef<'b> {
            fn eq(&self, other: &PhysStateRef<'_>) -> bool {
                self.pos() == other.pos()
                    && self.rot_mat() == other.rot_mat()
                    && self.vel() == other.vel()
                    && self.ang_vel() == other.ang_vel()
            }
        }

        impl<'a, 'b> ::core::cmp::PartialOrd<PhysStateRef<'a>> for PhysStateRef<'b> {
            fn partial_cmp(&self, other: &PhysStateRef<'_>) -> ::core::option::Option<::core::cmp::Ordering> {
                match self.pos().partial_cmp(&other.pos()) {
                    ::core::option::Option::Some(::core::cmp::Ordering::Equal) => (),
                    o => return o,
                }

                match self.rot_mat().partial_cmp(&other.rot_mat()) {
                    ::core::option::Option::Some(::core::cmp::Ordering::Equal) => (),
                    o => return o,
                }

                match self.vel().partial_cmp(&other.vel()) {
                    ::core::option::Option::Some(::core::cmp::Ordering::Equal) => (),
                    o => return o,
                }

                self.ang_vel().partial_cmp(&other.ang_vel())
            }
        }

        impl<'a> ::planus::TableRead<'a> for PhysStateRef<'a> {
            #[inline]
            fn from_buffer(
                buffer: ::planus::SliceWithStartOffset<'a>,
                offset: usize,
            ) -> ::core::result::Result<Self, ::planus::errors::ErrorKind> {
                let buffer = buffer.advance_as_array::<72>(offset)?;
                ::core::result::Result::Ok(Self(buffer))
            }
        }

        impl<'a> ::planus::VectorRead<'a> for PhysStateRef<'a> {
            const STRIDE: usize = 72;

            #[inline]
            unsafe fn from_buffer(buffer: ::planus::SliceWithStartOffset<'a>, offset: usize) -> Self {
                Self(unsafe { buffer.unchecked_advance_as_array(offset) })
            }
        }

        /// # Safety
        /// The planus compiler generates implementations that initialize
        /// the bytes in `write_values`.
        unsafe impl ::planus::VectorWrite<PhysState> for PhysState {
            const STRIDE: usize = 72;

            type Value = PhysState;

            #[inline]
            fn prepare(&self, _builder: &mut ::planus::Builder) -> Self::Value {
                *self
            }

            #[inline]
            unsafe fn write_values(values: &[PhysState], bytes: *mut ::core::mem::MaybeUninit<u8>, buffer_position: u32) {
                let bytes = bytes as *mut [::core::mem::MaybeUninit<u8>; 72];
                for (i, v) in ::core::iter::Iterator::enumerate(values.iter()) {
                    ::planus::WriteAsPrimitive::write(
                        v,
                        ::planus::Cursor::new(unsafe { &mut *bytes.add(i) }),
                        buffer_position - (72 * i) as u32,
                    );
                }
            }
        }

        /// The struct `CarControls` in the namespace `rocketsim`
        ///
        /// Generated from these locations:
        /// * Struct `CarControls` in the file `spec/game_state.fbs:39`
        #[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Default, ::serde::Serialize, ::serde::Deserialize)]
        pub struct CarControls {
            /// The field `throttle` in the struct `CarControls`
            pub throttle: f32,

            /// The field `steer` in the struct `CarControls`
            pub steer: f32,

            /// The field `pitch` in the struct `CarControls`
            pub pitch: f32,

            /// The field `yaw` in the struct `CarControls`
            pub yaw: f32,

            /// The field `roll` in the struct `CarControls`
            pub roll: f32,

            /// The field `jump` in the struct `CarControls`
            pub jump: bool,

            /// The field `boost` in the struct `CarControls`
            pub boost: bool,

            /// The field `handbrake` in the struct `CarControls`
            pub handbrake: bool,
        }

        /// # Safety
        /// The Planus compiler correctly calculates `ALIGNMENT` and `SIZE`.
        unsafe impl ::planus::Primitive for CarControls {
            const ALIGNMENT: usize = 4;
            const SIZE: usize = 24;
        }

        #[allow(clippy::identity_op)]
        impl ::planus::WriteAsPrimitive<CarControls> for CarControls {
            #[inline]
            fn write<const N: usize>(&self, cursor: ::planus::Cursor<'_, N>, buffer_position: u32) {
                let (cur, cursor) = cursor.split::<4, 20>();
                self.throttle.write(cur, buffer_position - 0);
                let (cur, cursor) = cursor.split::<4, 16>();
                self.steer.write(cur, buffer_position - 4);
                let (cur, cursor) = cursor.split::<4, 12>();
                self.pitch.write(cur, buffer_position - 8);
                let (cur, cursor) = cursor.split::<4, 8>();
                self.yaw.write(cur, buffer_position - 12);
                let (cur, cursor) = cursor.split::<4, 4>();
                self.roll.write(cur, buffer_position - 16);
                let (cur, cursor) = cursor.split::<1, 3>();
                self.jump.write(cur, buffer_position - 20);
                let (cur, cursor) = cursor.split::<1, 2>();
                self.boost.write(cur, buffer_position - 21);
                let (cur, cursor) = cursor.split::<1, 1>();
                self.handbrake.write(cur, buffer_position - 22);
                let cursor = cursor.write::<1, 0>([0; 1]);
                cursor.finish([]);
            }
        }

        impl ::planus::WriteAsOffset<CarControls> for CarControls {
            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<CarControls> {
                unsafe {
                    builder.write_with(24, 3, |buffer_position, bytes| {
                        let bytes = bytes.as_mut_ptr();

                        ::planus::WriteAsPrimitive::write(
                            self,
                            ::planus::Cursor::new(&mut *(bytes as *mut [::core::mem::MaybeUninit<u8>; 24])),
                            buffer_position,
                        );
                    });
                }
                builder.current_offset()
            }
        }

        impl ::planus::WriteAs<CarControls> for CarControls {
            type Prepared = Self;
            #[inline]
            fn prepare(&self, _builder: &mut ::planus::Builder) -> Self {
                *self
            }
        }

        impl ::planus::WriteAsOptional<CarControls> for CarControls {
            type Prepared = Self;
            #[inline]
            fn prepare(&self, _builder: &mut ::planus::Builder) -> ::core::option::Option<Self> {
                ::core::option::Option::Some(*self)
            }
        }

        /// Reference to a deserialized [CarControls].
        #[derive(Copy, Clone)]
        pub struct CarControlsRef<'a>(::planus::ArrayWithStartOffset<'a, 24>);

        impl<'a> CarControlsRef<'a> {
            /// Getter for the [`throttle` field](CarControls#structfield.throttle).
            pub fn throttle(&self) -> f32 {
                let buffer = self.0.advance_as_array::<4>(0).unwrap();

                f32::from_le_bytes(*buffer.as_array())
            }

            /// Getter for the [`steer` field](CarControls#structfield.steer).
            pub fn steer(&self) -> f32 {
                let buffer = self.0.advance_as_array::<4>(4).unwrap();

                f32::from_le_bytes(*buffer.as_array())
            }

            /// Getter for the [`pitch` field](CarControls#structfield.pitch).
            pub fn pitch(&self) -> f32 {
                let buffer = self.0.advance_as_array::<4>(8).unwrap();

                f32::from_le_bytes(*buffer.as_array())
            }

            /// Getter for the [`yaw` field](CarControls#structfield.yaw).
            pub fn yaw(&self) -> f32 {
                let buffer = self.0.advance_as_array::<4>(12).unwrap();

                f32::from_le_bytes(*buffer.as_array())
            }

            /// Getter for the [`roll` field](CarControls#structfield.roll).
            pub fn roll(&self) -> f32 {
                let buffer = self.0.advance_as_array::<4>(16).unwrap();

                f32::from_le_bytes(*buffer.as_array())
            }

            /// Getter for the [`jump` field](CarControls#structfield.jump).
            pub fn jump(&self) -> bool {
                let buffer = self.0.advance_as_array::<1>(20).unwrap();

                buffer.as_array()[0] != 0
            }

            /// Getter for the [`boost` field](CarControls#structfield.boost).
            pub fn boost(&self) -> bool {
                let buffer = self.0.advance_as_array::<1>(21).unwrap();

                buffer.as_array()[0] != 0
            }

            /// Getter for the [`handbrake` field](CarControls#structfield.handbrake).
            pub fn handbrake(&self) -> bool {
                let buffer = self.0.advance_as_array::<1>(22).unwrap();

                buffer.as_array()[0] != 0
            }
        }

        impl<'a> ::core::fmt::Debug for CarControlsRef<'a> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                let mut f = f.debug_struct("CarControlsRef");
                f.field("throttle", &self.throttle());
                f.field("steer", &self.steer());
                f.field("pitch", &self.pitch());
                f.field("yaw", &self.yaw());
                f.field("roll", &self.roll());
                f.field("jump", &self.jump());
                f.field("boost", &self.boost());
                f.field("handbrake", &self.handbrake());
                f.finish()
            }
        }

        impl<'a> ::core::convert::From<::planus::ArrayWithStartOffset<'a, 24>> for CarControlsRef<'a> {
            fn from(array: ::planus::ArrayWithStartOffset<'a, 24>) -> Self {
                Self(array)
            }
        }

        impl<'a> ::core::convert::From<CarControlsRef<'a>> for CarControls {
            #[allow(unreachable_code)]
            fn from(value: CarControlsRef<'a>) -> Self {
                Self {
                    throttle: value.throttle(),
                    steer: value.steer(),
                    pitch: value.pitch(),
                    yaw: value.yaw(),
                    roll: value.roll(),
                    jump: value.jump(),
                    boost: value.boost(),
                    handbrake: value.handbrake(),
                }
            }
        }

        impl<'a, 'b> ::core::cmp::PartialEq<CarControlsRef<'a>> for CarControlsRef<'b> {
            fn eq(&self, other: &CarControlsRef<'_>) -> bool {
                self.throttle() == other.throttle()
                    && self.steer() == other.steer()
                    && self.pitch() == other.pitch()
                    && self.yaw() == other.yaw()
                    && self.roll() == other.roll()
                    && self.jump() == other.jump()
                    && self.boost() == other.boost()
                    && self.handbrake() == other.handbrake()
            }
        }

        impl<'a, 'b> ::core::cmp::PartialOrd<CarControlsRef<'a>> for CarControlsRef<'b> {
            fn partial_cmp(&self, other: &CarControlsRef<'_>) -> ::core::option::Option<::core::cmp::Ordering> {
                match self.throttle().partial_cmp(&other.throttle()) {
                    ::core::option::Option::Some(::core::cmp::Ordering::Equal) => (),
                    o => return o,
                }

                match self.steer().partial_cmp(&other.steer()) {
                    ::core::option::Option::Some(::core::cmp::Ordering::Equal) => (),
                    o => return o,
                }

                match self.pitch().partial_cmp(&other.pitch()) {
                    ::core::option::Option::Some(::core::cmp::Ordering::Equal) => (),
                    o => return o,
                }

                match self.yaw().partial_cmp(&other.yaw()) {
                    ::core::option::Option::Some(::core::cmp::Ordering::Equal) => (),
                    o => return o,
                }

                match self.roll().partial_cmp(&other.roll()) {
                    ::core::option::Option::Some(::core::cmp::Ordering::Equal) => (),
                    o => return o,
                }

                match self.jump().partial_cmp(&other.jump()) {
                    ::core::option::Option::Some(::core::cmp::Ordering::Equal) => (),
                    o => return o,
                }

                match self.boost().partial_cmp(&other.boost()) {
                    ::core::option::Option::Some(::core::cmp::Ordering::Equal) => (),
                    o => return o,
                }

                self.handbrake().partial_cmp(&other.handbrake())
            }
        }

        impl<'a> ::planus::TableRead<'a> for CarControlsRef<'a> {
            #[inline]
            fn from_buffer(
                buffer: ::planus::SliceWithStartOffset<'a>,
                offset: usize,
            ) -> ::core::result::Result<Self, ::planus::errors::ErrorKind> {
                let buffer = buffer.advance_as_array::<24>(offset)?;
                ::core::result::Result::Ok(Self(buffer))
            }
        }

        impl<'a> ::planus::VectorRead<'a> for CarControlsRef<'a> {
            const STRIDE: usize = 24;

            #[inline]
            unsafe fn from_buffer(buffer: ::planus::SliceWithStartOffset<'a>, offset: usize) -> Self {
                Self(unsafe { buffer.unchecked_advance_as_array(offset) })
            }
        }

        /// # Safety
        /// The planus compiler generates implementations that initialize
        /// the bytes in `write_values`.
        unsafe impl ::planus::VectorWrite<CarControls> for CarControls {
            const STRIDE: usize = 24;

            type Value = CarControls;

            #[inline]
            fn prepare(&self, _builder: &mut ::planus::Builder) -> Self::Value {
                *self
            }

            #[inline]
            unsafe fn write_values(values: &[CarControls], bytes: *mut ::core::mem::MaybeUninit<u8>, buffer_position: u32) {
                let bytes = bytes as *mut [::core::mem::MaybeUninit<u8>; 24];
                for (i, v) in ::core::iter::Iterator::enumerate(values.iter()) {
                    ::planus::WriteAsPrimitive::write(
                        v,
                        ::planus::Cursor::new(unsafe { &mut *bytes.add(i) }),
                        buffer_position - (24 * i) as u32,
                    );
                }
            }
        }

        /// The struct `WheelPairConfig` in the namespace `rocketsim`
        ///
        /// Generated from these locations:
        /// * Struct `WheelPairConfig` in the file `spec/game_state.fbs:50`
        #[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Default, ::serde::Serialize, ::serde::Deserialize)]
        pub struct WheelPairConfig {
            /// The field `wheel_radius` in the struct `WheelPairConfig`
            pub wheel_radius: f32,

            /// The field `suspension_rest_length` in the struct `WheelPairConfig`
            pub suspension_rest_length: f32,

            /// The field `connection_point_offset` in the struct `WheelPairConfig`
            pub connection_point_offset: self::Vec3,
        }

        /// # Safety
        /// The Planus compiler correctly calculates `ALIGNMENT` and `SIZE`.
        unsafe impl ::planus::Primitive for WheelPairConfig {
            const ALIGNMENT: usize = 4;
            const SIZE: usize = 20;
        }

        #[allow(clippy::identity_op)]
        impl ::planus::WriteAsPrimitive<WheelPairConfig> for WheelPairConfig {
            #[inline]
            fn write<const N: usize>(&self, cursor: ::planus::Cursor<'_, N>, buffer_position: u32) {
                let (cur, cursor) = cursor.split::<4, 16>();
                self.wheel_radius.write(cur, buffer_position - 0);
                let (cur, cursor) = cursor.split::<4, 12>();
                self.suspension_rest_length.write(cur, buffer_position - 4);
                let (cur, cursor) = cursor.split::<12, 0>();
                self.connection_point_offset.write(cur, buffer_position - 8);
                cursor.finish([]);
            }
        }

        impl ::planus::WriteAsOffset<WheelPairConfig> for WheelPairConfig {
            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<WheelPairConfig> {
                unsafe {
                    builder.write_with(20, 3, |buffer_position, bytes| {
                        let bytes = bytes.as_mut_ptr();

                        ::planus::WriteAsPrimitive::write(
                            self,
                            ::planus::Cursor::new(&mut *(bytes as *mut [::core::mem::MaybeUninit<u8>; 20])),
                            buffer_position,
                        );
                    });
                }
                builder.current_offset()
            }
        }

        impl ::planus::WriteAs<WheelPairConfig> for WheelPairConfig {
            type Prepared = Self;
            #[inline]
            fn prepare(&self, _builder: &mut ::planus::Builder) -> Self {
                *self
            }
        }

        impl ::planus::WriteAsOptional<WheelPairConfig> for WheelPairConfig {
            type Prepared = Self;
            #[inline]
            fn prepare(&self, _builder: &mut ::planus::Builder) -> ::core::option::Option<Self> {
                ::core::option::Option::Some(*self)
            }
        }

        /// Reference to a deserialized [WheelPairConfig].
        #[derive(Copy, Clone)]
        pub struct WheelPairConfigRef<'a>(::planus::ArrayWithStartOffset<'a, 20>);

        impl<'a> WheelPairConfigRef<'a> {
            /// Getter for the [`wheel_radius` field](WheelPairConfig#structfield.wheel_radius).
            pub fn wheel_radius(&self) -> f32 {
                let buffer = self.0.advance_as_array::<4>(0).unwrap();

                f32::from_le_bytes(*buffer.as_array())
            }

            /// Getter for the [`suspension_rest_length` field](WheelPairConfig#structfield.suspension_rest_length).
            pub fn suspension_rest_length(&self) -> f32 {
                let buffer = self.0.advance_as_array::<4>(4).unwrap();

                f32::from_le_bytes(*buffer.as_array())
            }

            /// Getter for the [`connection_point_offset` field](WheelPairConfig#structfield.connection_point_offset).
            pub fn connection_point_offset(&self) -> self::Vec3Ref<'a> {
                let buffer = self.0.advance_as_array::<12>(8).unwrap();

                ::core::convert::From::from(buffer)
            }
        }

        impl<'a> ::core::fmt::Debug for WheelPairConfigRef<'a> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                let mut f = f.debug_struct("WheelPairConfigRef");
                f.field("wheel_radius", &self.wheel_radius());
                f.field("suspension_rest_length", &self.suspension_rest_length());
                f.field("connection_point_offset", &self.connection_point_offset());
                f.finish()
            }
        }

        impl<'a> ::core::convert::From<::planus::ArrayWithStartOffset<'a, 20>> for WheelPairConfigRef<'a> {
            fn from(array: ::planus::ArrayWithStartOffset<'a, 20>) -> Self {
                Self(array)
            }
        }

        impl<'a> ::core::convert::From<WheelPairConfigRef<'a>> for WheelPairConfig {
            #[allow(unreachable_code)]
            fn from(value: WheelPairConfigRef<'a>) -> Self {
                Self {
                    wheel_radius: value.wheel_radius(),
                    suspension_rest_length: value.suspension_rest_length(),
                    connection_point_offset: ::core::convert::From::from(value.connection_point_offset()),
                }
            }
        }

        impl<'a, 'b> ::core::cmp::PartialEq<WheelPairConfigRef<'a>> for WheelPairConfigRef<'b> {
            fn eq(&self, other: &WheelPairConfigRef<'_>) -> bool {
                self.wheel_radius() == other.wheel_radius()
                    && self.suspension_rest_length() == other.suspension_rest_length()
                    && self.connection_point_offset() == other.connection_point_offset()
            }
        }

        impl<'a, 'b> ::core::cmp::PartialOrd<WheelPairConfigRef<'a>> for WheelPairConfigRef<'b> {
            fn partial_cmp(&self, other: &WheelPairConfigRef<'_>) -> ::core::option::Option<::core::cmp::Ordering> {
                match self.wheel_radius().partial_cmp(&other.wheel_radius()) {
                    ::core::option::Option::Some(::core::cmp::Ordering::Equal) => (),
                    o => return o,
                }

                match self.suspension_rest_length().partial_cmp(&other.suspension_rest_length()) {
                    ::core::option::Option::Some(::core::cmp::Ordering::Equal) => (),
                    o => return o,
                }

                self.connection_point_offset().partial_cmp(&other.connection_point_offset())
            }
        }

        impl<'a> ::planus::TableRead<'a> for WheelPairConfigRef<'a> {
            #[inline]
            fn from_buffer(
                buffer: ::planus::SliceWithStartOffset<'a>,
                offset: usize,
            ) -> ::core::result::Result<Self, ::planus::errors::ErrorKind> {
                let buffer = buffer.advance_as_array::<20>(offset)?;
                ::core::result::Result::Ok(Self(buffer))
            }
        }

        impl<'a> ::planus::VectorRead<'a> for WheelPairConfigRef<'a> {
            const STRIDE: usize = 20;

            #[inline]
            unsafe fn from_buffer(buffer: ::planus::SliceWithStartOffset<'a>, offset: usize) -> Self {
                Self(unsafe { buffer.unchecked_advance_as_array(offset) })
            }
        }

        /// # Safety
        /// The planus compiler generates implementations that initialize
        /// the bytes in `write_values`.
        unsafe impl ::planus::VectorWrite<WheelPairConfig> for WheelPairConfig {
            const STRIDE: usize = 20;

            type Value = WheelPairConfig;

            #[inline]
            fn prepare(&self, _builder: &mut ::planus::Builder) -> Self::Value {
                *self
            }

            #[inline]
            unsafe fn write_values(
                values: &[WheelPairConfig],
                bytes: *mut ::core::mem::MaybeUninit<u8>,
                buffer_position: u32,
            ) {
                let bytes = bytes as *mut [::core::mem::MaybeUninit<u8>; 20];
                for (i, v) in ::core::iter::Iterator::enumerate(values.iter()) {
                    ::planus::WriteAsPrimitive::write(
                        v,
                        ::planus::Cursor::new(unsafe { &mut *bytes.add(i) }),
                        buffer_position - (20 * i) as u32,
                    );
                }
            }
        }

        /// The struct `CarConfig` in the namespace `rocketsim`
        ///
        /// Generated from these locations:
        /// * Struct `CarConfig` in the file `spec/game_state.fbs:56`
        #[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Default, ::serde::Serialize, ::serde::Deserialize)]
        pub struct CarConfig {
            /// The field `hitbox_size` in the struct `CarConfig`
            pub hitbox_size: self::Vec3,

            /// The field `hitbox_pos_offset` in the struct `CarConfig`
            pub hitbox_pos_offset: self::Vec3,

            /// The field `front_wheels` in the struct `CarConfig`
            pub front_wheels: self::WheelPairConfig,

            /// The field `back_wheels` in the struct `CarConfig`
            pub back_wheels: self::WheelPairConfig,

            /// The field `three_wheels` in the struct `CarConfig`
            pub three_wheels: bool,

            /// The field `dodge_deadzone` in the struct `CarConfig`
            pub dodge_deadzone: f32,
        }

        /// # Safety
        /// The Planus compiler correctly calculates `ALIGNMENT` and `SIZE`.
        unsafe impl ::planus::Primitive for CarConfig {
            const ALIGNMENT: usize = 4;
            const SIZE: usize = 72;
        }

        #[allow(clippy::identity_op)]
        impl ::planus::WriteAsPrimitive<CarConfig> for CarConfig {
            #[inline]
            fn write<const N: usize>(&self, cursor: ::planus::Cursor<'_, N>, buffer_position: u32) {
                let (cur, cursor) = cursor.split::<12, 60>();
                self.hitbox_size.write(cur, buffer_position - 0);
                let (cur, cursor) = cursor.split::<12, 48>();
                self.hitbox_pos_offset.write(cur, buffer_position - 12);
                let (cur, cursor) = cursor.split::<20, 28>();
                self.front_wheels.write(cur, buffer_position - 24);
                let (cur, cursor) = cursor.split::<20, 8>();
                self.back_wheels.write(cur, buffer_position - 44);
                let (cur, cursor) = cursor.split::<1, 7>();
                self.three_wheels.write(cur, buffer_position - 64);
                let cursor = cursor.write::<3, 4>([0; 3]);
                let (cur, cursor) = cursor.split::<4, 0>();
                self.dodge_deadzone.write(cur, buffer_position - 68);
                cursor.finish([]);
            }
        }

        impl ::planus::WriteAsOffset<CarConfig> for CarConfig {
            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<CarConfig> {
                unsafe {
                    builder.write_with(72, 3, |buffer_position, bytes| {
                        let bytes = bytes.as_mut_ptr();

                        ::planus::WriteAsPrimitive::write(
                            self,
                            ::planus::Cursor::new(&mut *(bytes as *mut [::core::mem::MaybeUninit<u8>; 72])),
                            buffer_position,
                        );
                    });
                }
                builder.current_offset()
            }
        }

        impl ::planus::WriteAs<CarConfig> for CarConfig {
            type Prepared = Self;
            #[inline]
            fn prepare(&self, _builder: &mut ::planus::Builder) -> Self {
                *self
            }
        }

        impl ::planus::WriteAsOptional<CarConfig> for CarConfig {
            type Prepared = Self;
            #[inline]
            fn prepare(&self, _builder: &mut ::planus::Builder) -> ::core::option::Option<Self> {
                ::core::option::Option::Some(*self)
            }
        }

        /// Reference to a deserialized [CarConfig].
        #[derive(Copy, Clone)]
        pub struct CarConfigRef<'a>(::planus::ArrayWithStartOffset<'a, 72>);

        impl<'a> CarConfigRef<'a> {
            /// Getter for the [`hitbox_size` field](CarConfig#structfield.hitbox_size).
            pub fn hitbox_size(&self) -> self::Vec3Ref<'a> {
                let buffer = self.0.advance_as_array::<12>(0).unwrap();

                ::core::convert::From::from(buffer)
            }

            /// Getter for the [`hitbox_pos_offset` field](CarConfig#structfield.hitbox_pos_offset).
            pub fn hitbox_pos_offset(&self) -> self::Vec3Ref<'a> {
                let buffer = self.0.advance_as_array::<12>(12).unwrap();

                ::core::convert::From::from(buffer)
            }

            /// Getter for the [`front_wheels` field](CarConfig#structfield.front_wheels).
            pub fn front_wheels(&self) -> self::WheelPairConfigRef<'a> {
                let buffer = self.0.advance_as_array::<20>(24).unwrap();

                ::core::convert::From::from(buffer)
            }

            /// Getter for the [`back_wheels` field](CarConfig#structfield.back_wheels).
            pub fn back_wheels(&self) -> self::WheelPairConfigRef<'a> {
                let buffer = self.0.advance_as_array::<20>(44).unwrap();

                ::core::convert::From::from(buffer)
            }

            /// Getter for the [`three_wheels` field](CarConfig#structfield.three_wheels).
            pub fn three_wheels(&self) -> bool {
                let buffer = self.0.advance_as_array::<1>(64).unwrap();

                buffer.as_array()[0] != 0
            }

            /// Getter for the [`dodge_deadzone` field](CarConfig#structfield.dodge_deadzone).
            pub fn dodge_deadzone(&self) -> f32 {
                let buffer = self.0.advance_as_array::<4>(68).unwrap();

                f32::from_le_bytes(*buffer.as_array())
            }
        }

        impl<'a> ::core::fmt::Debug for CarConfigRef<'a> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                let mut f = f.debug_struct("CarConfigRef");
                f.field("hitbox_size", &self.hitbox_size());
                f.field("hitbox_pos_offset", &self.hitbox_pos_offset());
                f.field("front_wheels", &self.front_wheels());
                f.field("back_wheels", &self.back_wheels());
                f.field("three_wheels", &self.three_wheels());
                f.field("dodge_deadzone", &self.dodge_deadzone());
                f.finish()
            }
        }

        impl<'a> ::core::convert::From<::planus::ArrayWithStartOffset<'a, 72>> for CarConfigRef<'a> {
            fn from(array: ::planus::ArrayWithStartOffset<'a, 72>) -> Self {
                Self(array)
            }
        }

        impl<'a> ::core::convert::From<CarConfigRef<'a>> for CarConfig {
            #[allow(unreachable_code)]
            fn from(value: CarConfigRef<'a>) -> Self {
                Self {
                    hitbox_size: ::core::convert::From::from(value.hitbox_size()),
                    hitbox_pos_offset: ::core::convert::From::from(value.hitbox_pos_offset()),
                    front_wheels: ::core::convert::From::from(value.front_wheels()),
                    back_wheels: ::core::convert::From::from(value.back_wheels()),
                    three_wheels: value.three_wheels(),
                    dodge_deadzone: value.dodge_deadzone(),
                }
            }
        }

        impl<'a, 'b> ::core::cmp::PartialEq<CarConfigRef<'a>> for CarConfigRef<'b> {
            fn eq(&self, other: &CarConfigRef<'_>) -> bool {
                self.hitbox_size() == other.hitbox_size()
                    && self.hitbox_pos_offset() == other.hitbox_pos_offset()
                    && self.front_wheels() == other.front_wheels()
                    && self.back_wheels() == other.back_wheels()
                    && self.three_wheels() == other.three_wheels()
                    && self.dodge_deadzone() == other.dodge_deadzone()
            }
        }

        impl<'a, 'b> ::core::cmp::PartialOrd<CarConfigRef<'a>> for CarConfigRef<'b> {
            fn partial_cmp(&self, other: &CarConfigRef<'_>) -> ::core::option::Option<::core::cmp::Ordering> {
                match self.hitbox_size().partial_cmp(&other.hitbox_size()) {
                    ::core::option::Option::Some(::core::cmp::Ordering::Equal) => (),
                    o => return o,
                }

                match self.hitbox_pos_offset().partial_cmp(&other.hitbox_pos_offset()) {
                    ::core::option::Option::Some(::core::cmp::Ordering::Equal) => (),
                    o => return o,
                }

                match self.front_wheels().partial_cmp(&other.front_wheels()) {
                    ::core::option::Option::Some(::core::cmp::Ordering::Equal) => (),
                    o => return o,
                }

                match self.back_wheels().partial_cmp(&other.back_wheels()) {
                    ::core::option::Option::Some(::core::cmp::Ordering::Equal) => (),
                    o => return o,
                }

                match self.three_wheels().partial_cmp(&other.three_wheels()) {
                    ::core::option::Option::Some(::core::cmp::Ordering::Equal) => (),
                    o => return o,
                }

                self.dodge_deadzone().partial_cmp(&other.dodge_deadzone())
            }
        }

        impl<'a> ::planus::TableRead<'a> for CarConfigRef<'a> {
            #[inline]
            fn from_buffer(
                buffer: ::planus::SliceWithStartOffset<'a>,
                offset: usize,
            ) -> ::core::result::Result<Self, ::planus::errors::ErrorKind> {
                let buffer = buffer.advance_as_array::<72>(offset)?;
                ::core::result::Result::Ok(Self(buffer))
            }
        }

        impl<'a> ::planus::VectorRead<'a> for CarConfigRef<'a> {
            const STRIDE: usize = 72;

            #[inline]
            unsafe fn from_buffer(buffer: ::planus::SliceWithStartOffset<'a>, offset: usize) -> Self {
                Self(unsafe { buffer.unchecked_advance_as_array(offset) })
            }
        }

        /// # Safety
        /// The planus compiler generates implementations that initialize
        /// the bytes in `write_values`.
        unsafe impl ::planus::VectorWrite<CarConfig> for CarConfig {
            const STRIDE: usize = 72;

            type Value = CarConfig;

            #[inline]
            fn prepare(&self, _builder: &mut ::planus::Builder) -> Self::Value {
                *self
            }

            #[inline]
            unsafe fn write_values(values: &[CarConfig], bytes: *mut ::core::mem::MaybeUninit<u8>, buffer_position: u32) {
                let bytes = bytes as *mut [::core::mem::MaybeUninit<u8>; 72];
                for (i, v) in ::core::iter::Iterator::enumerate(values.iter()) {
                    ::planus::WriteAsPrimitive::write(
                        v,
                        ::planus::Cursor::new(unsafe { &mut *bytes.add(i) }),
                        buffer_position - (72 * i) as u32,
                    );
                }
            }
        }

        /// The table `CarContact` in the namespace `rocketsim`
        ///
        /// Generated from these locations:
        /// * Table `CarContact` in the file `spec/game_state.fbs:65`
        #[derive(Clone, Debug, PartialEq, PartialOrd, ::serde::Serialize, ::serde::Deserialize)]
        pub struct CarContact {
            /// The field `other_car_id` in the table `CarContact`
            pub other_car_id: u64,
            /// The field `cooldown_timer` in the table `CarContact`
            pub cooldown_timer: f32,
        }

        #[allow(clippy::derivable_impls)]
        impl ::core::default::Default for CarContact {
            fn default() -> Self {
                Self {
                    other_car_id: 0,
                    cooldown_timer: 0.0,
                }
            }
        }

        impl CarContact {
            /// Creates a [CarContactBuilder] for serializing an instance of this table.
            #[inline]
            pub fn builder() -> CarContactBuilder<()> {
                CarContactBuilder(())
            }

            #[allow(clippy::too_many_arguments)]
            pub fn create(
                builder: &mut ::planus::Builder,
                field_other_car_id: impl ::planus::WriteAsDefault<u64, u64>,
                field_cooldown_timer: impl ::planus::WriteAsDefault<f32, f32>,
            ) -> ::planus::Offset<Self> {
                let prepared_other_car_id = field_other_car_id.prepare(builder, &0);
                let prepared_cooldown_timer = field_cooldown_timer.prepare(builder, &0.0);

                let mut table_writer: ::planus::table_writer::TableWriter<8> = ::core::default::Default::default();
                if prepared_other_car_id.is_some() {
                    table_writer.write_entry::<u64>(0);
                }
                if prepared_cooldown_timer.is_some() {
                    table_writer.write_entry::<f32>(1);
                }

                unsafe {
                    table_writer.finish(builder, |object_writer| {
                        if let ::core::option::Option::Some(prepared_other_car_id) = prepared_other_car_id {
                            object_writer.write::<_, _, 8>(&prepared_other_car_id);
                        }
                        if let ::core::option::Option::Some(prepared_cooldown_timer) = prepared_cooldown_timer {
                            object_writer.write::<_, _, 4>(&prepared_cooldown_timer);
                        }
                    });
                }
                builder.current_offset()
            }
        }

        impl ::planus::WriteAs<::planus::Offset<CarContact>> for CarContact {
            type Prepared = ::planus::Offset<Self>;

            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<CarContact> {
                ::planus::WriteAsOffset::prepare(self, builder)
            }
        }

        impl ::planus::WriteAsOptional<::planus::Offset<CarContact>> for CarContact {
            type Prepared = ::planus::Offset<Self>;

            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::core::option::Option<::planus::Offset<CarContact>> {
                ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
            }
        }

        impl ::planus::WriteAsOffset<CarContact> for CarContact {
            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<CarContact> {
                CarContact::create(builder, self.other_car_id, self.cooldown_timer)
            }
        }

        /// Builder for serializing an instance of the [CarContact] type.
        ///
        /// Can be created using the [CarContact::builder] method.
        #[derive(Debug)]
        #[must_use]
        pub struct CarContactBuilder<State>(State);

        impl CarContactBuilder<()> {
            /// Setter for the [`other_car_id` field](CarContact#structfield.other_car_id).
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn other_car_id<T0>(self, value: T0) -> CarContactBuilder<(T0,)>
            where
                T0: ::planus::WriteAsDefault<u64, u64>,
            {
                CarContactBuilder((value,))
            }

            /// Sets the [`other_car_id` field](CarContact#structfield.other_car_id) to the default value.
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn other_car_id_as_default(self) -> CarContactBuilder<(::planus::DefaultValue,)> {
                self.other_car_id(::planus::DefaultValue)
            }
        }

        impl<T0> CarContactBuilder<(T0,)> {
            /// Setter for the [`cooldown_timer` field](CarContact#structfield.cooldown_timer).
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn cooldown_timer<T1>(self, value: T1) -> CarContactBuilder<(T0, T1)>
            where
                T1: ::planus::WriteAsDefault<f32, f32>,
            {
                let (v0,) = self.0;
                CarContactBuilder((v0, value))
            }

            /// Sets the [`cooldown_timer` field](CarContact#structfield.cooldown_timer) to the default value.
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn cooldown_timer_as_default(self) -> CarContactBuilder<(T0, ::planus::DefaultValue)> {
                self.cooldown_timer(::planus::DefaultValue)
            }
        }

        impl<T0, T1> CarContactBuilder<(T0, T1)> {
            /// Finish writing the builder to get an [Offset](::planus::Offset) to a serialized [CarContact].
            #[inline]
            pub fn finish(self, builder: &mut ::planus::Builder) -> ::planus::Offset<CarContact>
            where
                Self: ::planus::WriteAsOffset<CarContact>,
            {
                ::planus::WriteAsOffset::prepare(&self, builder)
            }
        }

        impl<T0: ::planus::WriteAsDefault<u64, u64>, T1: ::planus::WriteAsDefault<f32, f32>>
            ::planus::WriteAs<::planus::Offset<CarContact>> for CarContactBuilder<(T0, T1)>
        {
            type Prepared = ::planus::Offset<CarContact>;

            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<CarContact> {
                ::planus::WriteAsOffset::prepare(self, builder)
            }
        }

        impl<T0: ::planus::WriteAsDefault<u64, u64>, T1: ::planus::WriteAsDefault<f32, f32>>
            ::planus::WriteAsOptional<::planus::Offset<CarContact>> for CarContactBuilder<(T0, T1)>
        {
            type Prepared = ::planus::Offset<CarContact>;

            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::core::option::Option<::planus::Offset<CarContact>> {
                ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
            }
        }

        impl<T0: ::planus::WriteAsDefault<u64, u64>, T1: ::planus::WriteAsDefault<f32, f32>>
            ::planus::WriteAsOffset<CarContact> for CarContactBuilder<(T0, T1)>
        {
            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<CarContact> {
                let (v0, v1) = &self.0;
                CarContact::create(builder, v0, v1)
            }
        }

        /// Reference to a deserialized [CarContact].
        #[derive(Copy, Clone)]
        pub struct CarContactRef<'a>(#[allow(dead_code)] ::planus::table_reader::Table<'a>);

        impl<'a> CarContactRef<'a> {
            /// Getter for the [`other_car_id` field](CarContact#structfield.other_car_id).
            #[inline]
            pub fn other_car_id(&self) -> ::planus::Result<u64> {
                ::core::result::Result::Ok(self.0.access(0, "CarContact", "other_car_id")?.unwrap_or(0))
            }

            /// Getter for the [`cooldown_timer` field](CarContact#structfield.cooldown_timer).
            #[inline]
            pub fn cooldown_timer(&self) -> ::planus::Result<f32> {
                ::core::result::Result::Ok(self.0.access(1, "CarContact", "cooldown_timer")?.unwrap_or(0.0))
            }
        }

        impl<'a> ::core::fmt::Debug for CarContactRef<'a> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                let mut f = f.debug_struct("CarContactRef");
                f.field("other_car_id", &self.other_car_id());
                f.field("cooldown_timer", &self.cooldown_timer());
                f.finish()
            }
        }

        impl<'a> ::core::convert::TryFrom<CarContactRef<'a>> for CarContact {
            type Error = ::planus::Error;

            #[allow(unreachable_code)]
            fn try_from(value: CarContactRef<'a>) -> ::planus::Result<Self> {
                ::core::result::Result::Ok(Self {
                    other_car_id: ::core::convert::TryInto::try_into(value.other_car_id()?)?,
                    cooldown_timer: ::core::convert::TryInto::try_into(value.cooldown_timer()?)?,
                })
            }
        }

        impl<'a> ::planus::TableRead<'a> for CarContactRef<'a> {
            #[inline]
            fn from_buffer(
                buffer: ::planus::SliceWithStartOffset<'a>,
                offset: usize,
            ) -> ::core::result::Result<Self, ::planus::errors::ErrorKind> {
                ::core::result::Result::Ok(Self(::planus::table_reader::Table::from_buffer(buffer, offset)?))
            }
        }

        impl<'a> ::planus::VectorReadInner<'a> for CarContactRef<'a> {
            type Error = ::planus::Error;
            const STRIDE: usize = 4;

            unsafe fn from_buffer(buffer: ::planus::SliceWithStartOffset<'a>, offset: usize) -> ::planus::Result<Self> {
                ::planus::TableRead::from_buffer(buffer, offset)
                    .map_err(|error_kind| error_kind.with_error_location("[CarContactRef]", "get", buffer.offset_from_start))
            }
        }

        /// # Safety
        /// The planus compiler generates implementations that initialize
        /// the bytes in `write_values`.
        unsafe impl ::planus::VectorWrite<::planus::Offset<CarContact>> for CarContact {
            type Value = ::planus::Offset<CarContact>;
            const STRIDE: usize = 4;
            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> Self::Value {
                ::planus::WriteAs::prepare(self, builder)
            }

            #[inline]
            unsafe fn write_values(
                values: &[::planus::Offset<CarContact>],
                bytes: *mut ::core::mem::MaybeUninit<u8>,
                buffer_position: u32,
            ) {
                let bytes = bytes as *mut [::core::mem::MaybeUninit<u8>; 4];
                for (i, v) in ::core::iter::Iterator::enumerate(values.iter()) {
                    ::planus::WriteAsPrimitive::write(
                        v,
                        ::planus::Cursor::new(unsafe { &mut *bytes.add(i) }),
                        buffer_position - (Self::STRIDE * i) as u32,
                    );
                }
            }
        }

        impl<'a> ::planus::ReadAsRoot<'a> for CarContactRef<'a> {
            fn read_as_root(slice: &'a [u8]) -> ::planus::Result<Self> {
                ::planus::TableRead::from_buffer(
                    ::planus::SliceWithStartOffset {
                        buffer: slice,
                        offset_from_start: 0,
                    },
                    0,
                )
                .map_err(|error_kind| error_kind.with_error_location("[CarContactRef]", "read_as_root", 0))
            }
        }

        /// The table `BallHitInfo` in the namespace `rocketsim`
        ///
        /// Generated from these locations:
        /// * Table `BallHitInfo` in the file `spec/game_state.fbs:70`
        #[derive(Clone, Debug, PartialEq, PartialOrd, ::serde::Serialize, ::serde::Deserialize)]
        pub struct BallHitInfo {
            /// The field `relative_pos_on_ball` in the table `BallHitInfo`
            pub relative_pos_on_ball: self::Vec3,
            /// The field `ball_pos` in the table `BallHitInfo`
            pub ball_pos: self::Vec3,
            /// The field `extra_hit_vel` in the table `BallHitInfo`
            pub extra_hit_vel: self::Vec3,
            /// The field `tick_count_when_hit` in the table `BallHitInfo`
            pub tick_count_when_hit: u64,
            /// The field `tick_count_when_extra_impulse_applied` in the table `BallHitInfo`
            pub tick_count_when_extra_impulse_applied: u64,
        }

        #[allow(clippy::derivable_impls)]
        impl ::core::default::Default for BallHitInfo {
            fn default() -> Self {
                Self {
                    relative_pos_on_ball: ::core::default::Default::default(),
                    ball_pos: ::core::default::Default::default(),
                    extra_hit_vel: ::core::default::Default::default(),
                    tick_count_when_hit: 0,
                    tick_count_when_extra_impulse_applied: 0,
                }
            }
        }

        impl BallHitInfo {
            /// Creates a [BallHitInfoBuilder] for serializing an instance of this table.
            #[inline]
            pub fn builder() -> BallHitInfoBuilder<()> {
                BallHitInfoBuilder(())
            }

            #[allow(clippy::too_many_arguments)]
            pub fn create(
                builder: &mut ::planus::Builder,
                field_relative_pos_on_ball: impl ::planus::WriteAs<self::Vec3>,
                field_ball_pos: impl ::planus::WriteAs<self::Vec3>,
                field_extra_hit_vel: impl ::planus::WriteAs<self::Vec3>,
                field_tick_count_when_hit: impl ::planus::WriteAsDefault<u64, u64>,
                field_tick_count_when_extra_impulse_applied: impl ::planus::WriteAsDefault<u64, u64>,
            ) -> ::planus::Offset<Self> {
                let prepared_relative_pos_on_ball = field_relative_pos_on_ball.prepare(builder);
                let prepared_ball_pos = field_ball_pos.prepare(builder);
                let prepared_extra_hit_vel = field_extra_hit_vel.prepare(builder);
                let prepared_tick_count_when_hit = field_tick_count_when_hit.prepare(builder, &0);
                let prepared_tick_count_when_extra_impulse_applied =
                    field_tick_count_when_extra_impulse_applied.prepare(builder, &0);

                let mut table_writer: ::planus::table_writer::TableWriter<14> = ::core::default::Default::default();
                if prepared_tick_count_when_hit.is_some() {
                    table_writer.write_entry::<u64>(3);
                }
                if prepared_tick_count_when_extra_impulse_applied.is_some() {
                    table_writer.write_entry::<u64>(4);
                }
                table_writer.write_entry::<self::Vec3>(0);
                table_writer.write_entry::<self::Vec3>(1);
                table_writer.write_entry::<self::Vec3>(2);

                unsafe {
                    table_writer.finish(builder, |object_writer| {
                        if let ::core::option::Option::Some(prepared_tick_count_when_hit) = prepared_tick_count_when_hit {
                            object_writer.write::<_, _, 8>(&prepared_tick_count_when_hit);
                        }
                        if let ::core::option::Option::Some(prepared_tick_count_when_extra_impulse_applied) =
                            prepared_tick_count_when_extra_impulse_applied
                        {
                            object_writer.write::<_, _, 8>(&prepared_tick_count_when_extra_impulse_applied);
                        }
                        object_writer.write::<_, _, 12>(&prepared_relative_pos_on_ball);
                        object_writer.write::<_, _, 12>(&prepared_ball_pos);
                        object_writer.write::<_, _, 12>(&prepared_extra_hit_vel);
                    });
                }
                builder.current_offset()
            }
        }

        impl ::planus::WriteAs<::planus::Offset<BallHitInfo>> for BallHitInfo {
            type Prepared = ::planus::Offset<Self>;

            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<BallHitInfo> {
                ::planus::WriteAsOffset::prepare(self, builder)
            }
        }

        impl ::planus::WriteAsOptional<::planus::Offset<BallHitInfo>> for BallHitInfo {
            type Prepared = ::planus::Offset<Self>;

            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::core::option::Option<::planus::Offset<BallHitInfo>> {
                ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
            }
        }

        impl ::planus::WriteAsOffset<BallHitInfo> for BallHitInfo {
            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<BallHitInfo> {
                BallHitInfo::create(
                    builder,
                    self.relative_pos_on_ball,
                    self.ball_pos,
                    self.extra_hit_vel,
                    self.tick_count_when_hit,
                    self.tick_count_when_extra_impulse_applied,
                )
            }
        }

        /// Builder for serializing an instance of the [BallHitInfo] type.
        ///
        /// Can be created using the [BallHitInfo::builder] method.
        #[derive(Debug)]
        #[must_use]
        pub struct BallHitInfoBuilder<State>(State);

        impl BallHitInfoBuilder<()> {
            /// Setter for the [`relative_pos_on_ball` field](BallHitInfo#structfield.relative_pos_on_ball).
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn relative_pos_on_ball<T0>(self, value: T0) -> BallHitInfoBuilder<(T0,)>
            where
                T0: ::planus::WriteAs<self::Vec3>,
            {
                BallHitInfoBuilder((value,))
            }
        }

        impl<T0> BallHitInfoBuilder<(T0,)> {
            /// Setter for the [`ball_pos` field](BallHitInfo#structfield.ball_pos).
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn ball_pos<T1>(self, value: T1) -> BallHitInfoBuilder<(T0, T1)>
            where
                T1: ::planus::WriteAs<self::Vec3>,
            {
                let (v0,) = self.0;
                BallHitInfoBuilder((v0, value))
            }
        }

        impl<T0, T1> BallHitInfoBuilder<(T0, T1)> {
            /// Setter for the [`extra_hit_vel` field](BallHitInfo#structfield.extra_hit_vel).
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn extra_hit_vel<T2>(self, value: T2) -> BallHitInfoBuilder<(T0, T1, T2)>
            where
                T2: ::planus::WriteAs<self::Vec3>,
            {
                let (v0, v1) = self.0;
                BallHitInfoBuilder((v0, v1, value))
            }
        }

        impl<T0, T1, T2> BallHitInfoBuilder<(T0, T1, T2)> {
            /// Setter for the [`tick_count_when_hit` field](BallHitInfo#structfield.tick_count_when_hit).
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn tick_count_when_hit<T3>(self, value: T3) -> BallHitInfoBuilder<(T0, T1, T2, T3)>
            where
                T3: ::planus::WriteAsDefault<u64, u64>,
            {
                let (v0, v1, v2) = self.0;
                BallHitInfoBuilder((v0, v1, v2, value))
            }

            /// Sets the [`tick_count_when_hit` field](BallHitInfo#structfield.tick_count_when_hit) to the default value.
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn tick_count_when_hit_as_default(self) -> BallHitInfoBuilder<(T0, T1, T2, ::planus::DefaultValue)> {
                self.tick_count_when_hit(::planus::DefaultValue)
            }
        }

        impl<T0, T1, T2, T3> BallHitInfoBuilder<(T0, T1, T2, T3)> {
            /// Setter for the [`tick_count_when_extra_impulse_applied` field](BallHitInfo#structfield.tick_count_when_extra_impulse_applied).
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn tick_count_when_extra_impulse_applied<T4>(self, value: T4) -> BallHitInfoBuilder<(T0, T1, T2, T3, T4)>
            where
                T4: ::planus::WriteAsDefault<u64, u64>,
            {
                let (v0, v1, v2, v3) = self.0;
                BallHitInfoBuilder((v0, v1, v2, v3, value))
            }

            /// Sets the [`tick_count_when_extra_impulse_applied` field](BallHitInfo#structfield.tick_count_when_extra_impulse_applied) to the default value.
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn tick_count_when_extra_impulse_applied_as_default(
                self,
            ) -> BallHitInfoBuilder<(T0, T1, T2, T3, ::planus::DefaultValue)> {
                self.tick_count_when_extra_impulse_applied(::planus::DefaultValue)
            }
        }

        impl<T0, T1, T2, T3, T4> BallHitInfoBuilder<(T0, T1, T2, T3, T4)> {
            /// Finish writing the builder to get an [Offset](::planus::Offset) to a serialized [BallHitInfo].
            #[inline]
            pub fn finish(self, builder: &mut ::planus::Builder) -> ::planus::Offset<BallHitInfo>
            where
                Self: ::planus::WriteAsOffset<BallHitInfo>,
            {
                ::planus::WriteAsOffset::prepare(&self, builder)
            }
        }

        impl<
            T0: ::planus::WriteAs<self::Vec3>,
            T1: ::planus::WriteAs<self::Vec3>,
            T2: ::planus::WriteAs<self::Vec3>,
            T3: ::planus::WriteAsDefault<u64, u64>,
            T4: ::planus::WriteAsDefault<u64, u64>,
        > ::planus::WriteAs<::planus::Offset<BallHitInfo>> for BallHitInfoBuilder<(T0, T1, T2, T3, T4)>
        {
            type Prepared = ::planus::Offset<BallHitInfo>;

            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<BallHitInfo> {
                ::planus::WriteAsOffset::prepare(self, builder)
            }
        }

        impl<
            T0: ::planus::WriteAs<self::Vec3>,
            T1: ::planus::WriteAs<self::Vec3>,
            T2: ::planus::WriteAs<self::Vec3>,
            T3: ::planus::WriteAsDefault<u64, u64>,
            T4: ::planus::WriteAsDefault<u64, u64>,
        > ::planus::WriteAsOptional<::planus::Offset<BallHitInfo>> for BallHitInfoBuilder<(T0, T1, T2, T3, T4)>
        {
            type Prepared = ::planus::Offset<BallHitInfo>;

            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::core::option::Option<::planus::Offset<BallHitInfo>> {
                ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
            }
        }

        impl<
            T0: ::planus::WriteAs<self::Vec3>,
            T1: ::planus::WriteAs<self::Vec3>,
            T2: ::planus::WriteAs<self::Vec3>,
            T3: ::planus::WriteAsDefault<u64, u64>,
            T4: ::planus::WriteAsDefault<u64, u64>,
        > ::planus::WriteAsOffset<BallHitInfo> for BallHitInfoBuilder<(T0, T1, T2, T3, T4)>
        {
            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<BallHitInfo> {
                let (v0, v1, v2, v3, v4) = &self.0;
                BallHitInfo::create(builder, v0, v1, v2, v3, v4)
            }
        }

        /// Reference to a deserialized [BallHitInfo].
        #[derive(Copy, Clone)]
        pub struct BallHitInfoRef<'a>(#[allow(dead_code)] ::planus::table_reader::Table<'a>);

        impl<'a> BallHitInfoRef<'a> {
            /// Getter for the [`relative_pos_on_ball` field](BallHitInfo#structfield.relative_pos_on_ball).
            #[inline]
            pub fn relative_pos_on_ball(&self) -> ::planus::Result<self::Vec3Ref<'a>> {
                self.0.access_required(0, "BallHitInfo", "relative_pos_on_ball")
            }

            /// Getter for the [`ball_pos` field](BallHitInfo#structfield.ball_pos).
            #[inline]
            pub fn ball_pos(&self) -> ::planus::Result<self::Vec3Ref<'a>> {
                self.0.access_required(1, "BallHitInfo", "ball_pos")
            }

            /// Getter for the [`extra_hit_vel` field](BallHitInfo#structfield.extra_hit_vel).
            #[inline]
            pub fn extra_hit_vel(&self) -> ::planus::Result<self::Vec3Ref<'a>> {
                self.0.access_required(2, "BallHitInfo", "extra_hit_vel")
            }

            /// Getter for the [`tick_count_when_hit` field](BallHitInfo#structfield.tick_count_when_hit).
            #[inline]
            pub fn tick_count_when_hit(&self) -> ::planus::Result<u64> {
                ::core::result::Result::Ok(self.0.access(3, "BallHitInfo", "tick_count_when_hit")?.unwrap_or(0))
            }

            /// Getter for the [`tick_count_when_extra_impulse_applied` field](BallHitInfo#structfield.tick_count_when_extra_impulse_applied).
            #[inline]
            pub fn tick_count_when_extra_impulse_applied(&self) -> ::planus::Result<u64> {
                ::core::result::Result::Ok(
                    self.0
                        .access(4, "BallHitInfo", "tick_count_when_extra_impulse_applied")?
                        .unwrap_or(0),
                )
            }
        }

        impl<'a> ::core::fmt::Debug for BallHitInfoRef<'a> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                let mut f = f.debug_struct("BallHitInfoRef");
                f.field("relative_pos_on_ball", &self.relative_pos_on_ball());
                f.field("ball_pos", &self.ball_pos());
                f.field("extra_hit_vel", &self.extra_hit_vel());
                f.field("tick_count_when_hit", &self.tick_count_when_hit());
                f.field(
                    "tick_count_when_extra_impulse_applied",
                    &self.tick_count_when_extra_impulse_applied(),
                );
                f.finish()
            }
        }

        impl<'a> ::core::convert::TryFrom<BallHitInfoRef<'a>> for BallHitInfo {
            type Error = ::planus::Error;

            #[allow(unreachable_code)]
            fn try_from(value: BallHitInfoRef<'a>) -> ::planus::Result<Self> {
                ::core::result::Result::Ok(Self {
                    relative_pos_on_ball: ::core::convert::Into::into(value.relative_pos_on_ball()?),
                    ball_pos: ::core::convert::Into::into(value.ball_pos()?),
                    extra_hit_vel: ::core::convert::Into::into(value.extra_hit_vel()?),
                    tick_count_when_hit: ::core::convert::TryInto::try_into(value.tick_count_when_hit()?)?,
                    tick_count_when_extra_impulse_applied: ::core::convert::TryInto::try_into(
                        value.tick_count_when_extra_impulse_applied()?,
                    )?,
                })
            }
        }

        impl<'a> ::planus::TableRead<'a> for BallHitInfoRef<'a> {
            #[inline]
            fn from_buffer(
                buffer: ::planus::SliceWithStartOffset<'a>,
                offset: usize,
            ) -> ::core::result::Result<Self, ::planus::errors::ErrorKind> {
                ::core::result::Result::Ok(Self(::planus::table_reader::Table::from_buffer(buffer, offset)?))
            }
        }

        impl<'a> ::planus::VectorReadInner<'a> for BallHitInfoRef<'a> {
            type Error = ::planus::Error;
            const STRIDE: usize = 4;

            unsafe fn from_buffer(buffer: ::planus::SliceWithStartOffset<'a>, offset: usize) -> ::planus::Result<Self> {
                ::planus::TableRead::from_buffer(buffer, offset).map_err(|error_kind| {
                    error_kind.with_error_location("[BallHitInfoRef]", "get", buffer.offset_from_start)
                })
            }
        }

        /// # Safety
        /// The planus compiler generates implementations that initialize
        /// the bytes in `write_values`.
        unsafe impl ::planus::VectorWrite<::planus::Offset<BallHitInfo>> for BallHitInfo {
            type Value = ::planus::Offset<BallHitInfo>;
            const STRIDE: usize = 4;
            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> Self::Value {
                ::planus::WriteAs::prepare(self, builder)
            }

            #[inline]
            unsafe fn write_values(
                values: &[::planus::Offset<BallHitInfo>],
                bytes: *mut ::core::mem::MaybeUninit<u8>,
                buffer_position: u32,
            ) {
                let bytes = bytes as *mut [::core::mem::MaybeUninit<u8>; 4];
                for (i, v) in ::core::iter::Iterator::enumerate(values.iter()) {
                    ::planus::WriteAsPrimitive::write(
                        v,
                        ::planus::Cursor::new(unsafe { &mut *bytes.add(i) }),
                        buffer_position - (Self::STRIDE * i) as u32,
                    );
                }
            }
        }

        impl<'a> ::planus::ReadAsRoot<'a> for BallHitInfoRef<'a> {
            fn read_as_root(slice: &'a [u8]) -> ::planus::Result<Self> {
                ::planus::TableRead::from_buffer(
                    ::planus::SliceWithStartOffset {
                        buffer: slice,
                        offset_from_start: 0,
                    },
                    0,
                )
                .map_err(|error_kind| error_kind.with_error_location("[BallHitInfoRef]", "read_as_root", 0))
            }
        }

        /// The struct `HeatseekerInfo` in the namespace `rocketsim`
        ///
        /// Generated from these locations:
        /// * Struct `HeatseekerInfo` in the file `spec/game_state.fbs:78`
        #[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Default, ::serde::Serialize, ::serde::Deserialize)]
        pub struct HeatseekerInfo {
            /// The field `y_target_dir` in the struct `HeatseekerInfo`
            pub y_target_dir: f32,

            /// The field `cur_target_speed` in the struct `HeatseekerInfo`
            pub cur_target_speed: f32,

            /// The field `time_since_hit` in the struct `HeatseekerInfo`
            pub time_since_hit: f32,
        }

        /// # Safety
        /// The Planus compiler correctly calculates `ALIGNMENT` and `SIZE`.
        unsafe impl ::planus::Primitive for HeatseekerInfo {
            const ALIGNMENT: usize = 4;
            const SIZE: usize = 12;
        }

        #[allow(clippy::identity_op)]
        impl ::planus::WriteAsPrimitive<HeatseekerInfo> for HeatseekerInfo {
            #[inline]
            fn write<const N: usize>(&self, cursor: ::planus::Cursor<'_, N>, buffer_position: u32) {
                let (cur, cursor) = cursor.split::<4, 8>();
                self.y_target_dir.write(cur, buffer_position - 0);
                let (cur, cursor) = cursor.split::<4, 4>();
                self.cur_target_speed.write(cur, buffer_position - 4);
                let (cur, cursor) = cursor.split::<4, 0>();
                self.time_since_hit.write(cur, buffer_position - 8);
                cursor.finish([]);
            }
        }

        impl ::planus::WriteAsOffset<HeatseekerInfo> for HeatseekerInfo {
            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<HeatseekerInfo> {
                unsafe {
                    builder.write_with(12, 3, |buffer_position, bytes| {
                        let bytes = bytes.as_mut_ptr();

                        ::planus::WriteAsPrimitive::write(
                            self,
                            ::planus::Cursor::new(&mut *(bytes as *mut [::core::mem::MaybeUninit<u8>; 12])),
                            buffer_position,
                        );
                    });
                }
                builder.current_offset()
            }
        }

        impl ::planus::WriteAs<HeatseekerInfo> for HeatseekerInfo {
            type Prepared = Self;
            #[inline]
            fn prepare(&self, _builder: &mut ::planus::Builder) -> Self {
                *self
            }
        }

        impl ::planus::WriteAsOptional<HeatseekerInfo> for HeatseekerInfo {
            type Prepared = Self;
            #[inline]
            fn prepare(&self, _builder: &mut ::planus::Builder) -> ::core::option::Option<Self> {
                ::core::option::Option::Some(*self)
            }
        }

        /// Reference to a deserialized [HeatseekerInfo].
        #[derive(Copy, Clone)]
        pub struct HeatseekerInfoRef<'a>(::planus::ArrayWithStartOffset<'a, 12>);

        impl<'a> HeatseekerInfoRef<'a> {
            /// Getter for the [`y_target_dir` field](HeatseekerInfo#structfield.y_target_dir).
            pub fn y_target_dir(&self) -> f32 {
                let buffer = self.0.advance_as_array::<4>(0).unwrap();

                f32::from_le_bytes(*buffer.as_array())
            }

            /// Getter for the [`cur_target_speed` field](HeatseekerInfo#structfield.cur_target_speed).
            pub fn cur_target_speed(&self) -> f32 {
                let buffer = self.0.advance_as_array::<4>(4).unwrap();

                f32::from_le_bytes(*buffer.as_array())
            }

            /// Getter for the [`time_since_hit` field](HeatseekerInfo#structfield.time_since_hit).
            pub fn time_since_hit(&self) -> f32 {
                let buffer = self.0.advance_as_array::<4>(8).unwrap();

                f32::from_le_bytes(*buffer.as_array())
            }
        }

        impl<'a> ::core::fmt::Debug for HeatseekerInfoRef<'a> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                let mut f = f.debug_struct("HeatseekerInfoRef");
                f.field("y_target_dir", &self.y_target_dir());
                f.field("cur_target_speed", &self.cur_target_speed());
                f.field("time_since_hit", &self.time_since_hit());
                f.finish()
            }
        }

        impl<'a> ::core::convert::From<::planus::ArrayWithStartOffset<'a, 12>> for HeatseekerInfoRef<'a> {
            fn from(array: ::planus::ArrayWithStartOffset<'a, 12>) -> Self {
                Self(array)
            }
        }

        impl<'a> ::core::convert::From<HeatseekerInfoRef<'a>> for HeatseekerInfo {
            #[allow(unreachable_code)]
            fn from(value: HeatseekerInfoRef<'a>) -> Self {
                Self {
                    y_target_dir: value.y_target_dir(),
                    cur_target_speed: value.cur_target_speed(),
                    time_since_hit: value.time_since_hit(),
                }
            }
        }

        impl<'a, 'b> ::core::cmp::PartialEq<HeatseekerInfoRef<'a>> for HeatseekerInfoRef<'b> {
            fn eq(&self, other: &HeatseekerInfoRef<'_>) -> bool {
                self.y_target_dir() == other.y_target_dir()
                    && self.cur_target_speed() == other.cur_target_speed()
                    && self.time_since_hit() == other.time_since_hit()
            }
        }

        impl<'a, 'b> ::core::cmp::PartialOrd<HeatseekerInfoRef<'a>> for HeatseekerInfoRef<'b> {
            fn partial_cmp(&self, other: &HeatseekerInfoRef<'_>) -> ::core::option::Option<::core::cmp::Ordering> {
                match self.y_target_dir().partial_cmp(&other.y_target_dir()) {
                    ::core::option::Option::Some(::core::cmp::Ordering::Equal) => (),
                    o => return o,
                }

                match self.cur_target_speed().partial_cmp(&other.cur_target_speed()) {
                    ::core::option::Option::Some(::core::cmp::Ordering::Equal) => (),
                    o => return o,
                }

                self.time_since_hit().partial_cmp(&other.time_since_hit())
            }
        }

        impl<'a> ::planus::TableRead<'a> for HeatseekerInfoRef<'a> {
            #[inline]
            fn from_buffer(
                buffer: ::planus::SliceWithStartOffset<'a>,
                offset: usize,
            ) -> ::core::result::Result<Self, ::planus::errors::ErrorKind> {
                let buffer = buffer.advance_as_array::<12>(offset)?;
                ::core::result::Result::Ok(Self(buffer))
            }
        }

        impl<'a> ::planus::VectorRead<'a> for HeatseekerInfoRef<'a> {
            const STRIDE: usize = 12;

            #[inline]
            unsafe fn from_buffer(buffer: ::planus::SliceWithStartOffset<'a>, offset: usize) -> Self {
                Self(unsafe { buffer.unchecked_advance_as_array(offset) })
            }
        }

        /// # Safety
        /// The planus compiler generates implementations that initialize
        /// the bytes in `write_values`.
        unsafe impl ::planus::VectorWrite<HeatseekerInfo> for HeatseekerInfo {
            const STRIDE: usize = 12;

            type Value = HeatseekerInfo;

            #[inline]
            fn prepare(&self, _builder: &mut ::planus::Builder) -> Self::Value {
                *self
            }

            #[inline]
            unsafe fn write_values(
                values: &[HeatseekerInfo],
                bytes: *mut ::core::mem::MaybeUninit<u8>,
                buffer_position: u32,
            ) {
                let bytes = bytes as *mut [::core::mem::MaybeUninit<u8>; 12];
                for (i, v) in ::core::iter::Iterator::enumerate(values.iter()) {
                    ::planus::WriteAsPrimitive::write(
                        v,
                        ::planus::Cursor::new(unsafe { &mut *bytes.add(i) }),
                        buffer_position - (12 * i) as u32,
                    );
                }
            }
        }

        /// The struct `DropshotInfo` in the namespace `rocketsim`
        ///
        /// Generated from these locations:
        /// * Struct `DropshotInfo` in the file `spec/game_state.fbs:84`
        #[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Default, ::serde::Serialize, ::serde::Deserialize)]
        pub struct DropshotInfo {
            /// The field `charge_level` in the struct `DropshotInfo`
            pub charge_level: i32,

            /// The field `accumulated_hit_force` in the struct `DropshotInfo`
            pub accumulated_hit_force: f32,

            /// The field `y_target_dir` in the struct `DropshotInfo`
            pub y_target_dir: f32,

            /// The field `has_damaged` in the struct `DropshotInfo`
            pub has_damaged: bool,

            /// The field `last_damage_tick` in the struct `DropshotInfo`
            pub last_damage_tick: u64,
        }

        /// # Safety
        /// The Planus compiler correctly calculates `ALIGNMENT` and `SIZE`.
        unsafe impl ::planus::Primitive for DropshotInfo {
            const ALIGNMENT: usize = 8;
            const SIZE: usize = 24;
        }

        #[allow(clippy::identity_op)]
        impl ::planus::WriteAsPrimitive<DropshotInfo> for DropshotInfo {
            #[inline]
            fn write<const N: usize>(&self, cursor: ::planus::Cursor<'_, N>, buffer_position: u32) {
                let (cur, cursor) = cursor.split::<4, 20>();
                self.charge_level.write(cur, buffer_position - 0);
                let (cur, cursor) = cursor.split::<4, 16>();
                self.accumulated_hit_force.write(cur, buffer_position - 4);
                let (cur, cursor) = cursor.split::<4, 12>();
                self.y_target_dir.write(cur, buffer_position - 8);
                let (cur, cursor) = cursor.split::<1, 11>();
                self.has_damaged.write(cur, buffer_position - 12);
                let cursor = cursor.write::<3, 8>([0; 3]);
                let (cur, cursor) = cursor.split::<8, 0>();
                self.last_damage_tick.write(cur, buffer_position - 16);
                cursor.finish([]);
            }
        }

        impl ::planus::WriteAsOffset<DropshotInfo> for DropshotInfo {
            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<DropshotInfo> {
                unsafe {
                    builder.write_with(24, 7, |buffer_position, bytes| {
                        let bytes = bytes.as_mut_ptr();

                        ::planus::WriteAsPrimitive::write(
                            self,
                            ::planus::Cursor::new(&mut *(bytes as *mut [::core::mem::MaybeUninit<u8>; 24])),
                            buffer_position,
                        );
                    });
                }
                builder.current_offset()
            }
        }

        impl ::planus::WriteAs<DropshotInfo> for DropshotInfo {
            type Prepared = Self;
            #[inline]
            fn prepare(&self, _builder: &mut ::planus::Builder) -> Self {
                *self
            }
        }

        impl ::planus::WriteAsOptional<DropshotInfo> for DropshotInfo {
            type Prepared = Self;
            #[inline]
            fn prepare(&self, _builder: &mut ::planus::Builder) -> ::core::option::Option<Self> {
                ::core::option::Option::Some(*self)
            }
        }

        /// Reference to a deserialized [DropshotInfo].
        #[derive(Copy, Clone)]
        pub struct DropshotInfoRef<'a>(::planus::ArrayWithStartOffset<'a, 24>);

        impl<'a> DropshotInfoRef<'a> {
            /// Getter for the [`charge_level` field](DropshotInfo#structfield.charge_level).
            pub fn charge_level(&self) -> i32 {
                let buffer = self.0.advance_as_array::<4>(0).unwrap();

                i32::from_le_bytes(*buffer.as_array())
            }

            /// Getter for the [`accumulated_hit_force` field](DropshotInfo#structfield.accumulated_hit_force).
            pub fn accumulated_hit_force(&self) -> f32 {
                let buffer = self.0.advance_as_array::<4>(4).unwrap();

                f32::from_le_bytes(*buffer.as_array())
            }

            /// Getter for the [`y_target_dir` field](DropshotInfo#structfield.y_target_dir).
            pub fn y_target_dir(&self) -> f32 {
                let buffer = self.0.advance_as_array::<4>(8).unwrap();

                f32::from_le_bytes(*buffer.as_array())
            }

            /// Getter for the [`has_damaged` field](DropshotInfo#structfield.has_damaged).
            pub fn has_damaged(&self) -> bool {
                let buffer = self.0.advance_as_array::<1>(12).unwrap();

                buffer.as_array()[0] != 0
            }

            /// Getter for the [`last_damage_tick` field](DropshotInfo#structfield.last_damage_tick).
            pub fn last_damage_tick(&self) -> u64 {
                let buffer = self.0.advance_as_array::<8>(16).unwrap();

                u64::from_le_bytes(*buffer.as_array())
            }
        }

        impl<'a> ::core::fmt::Debug for DropshotInfoRef<'a> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                let mut f = f.debug_struct("DropshotInfoRef");
                f.field("charge_level", &self.charge_level());
                f.field("accumulated_hit_force", &self.accumulated_hit_force());
                f.field("y_target_dir", &self.y_target_dir());
                f.field("has_damaged", &self.has_damaged());
                f.field("last_damage_tick", &self.last_damage_tick());
                f.finish()
            }
        }

        impl<'a> ::core::convert::From<::planus::ArrayWithStartOffset<'a, 24>> for DropshotInfoRef<'a> {
            fn from(array: ::planus::ArrayWithStartOffset<'a, 24>) -> Self {
                Self(array)
            }
        }

        impl<'a> ::core::convert::From<DropshotInfoRef<'a>> for DropshotInfo {
            #[allow(unreachable_code)]
            fn from(value: DropshotInfoRef<'a>) -> Self {
                Self {
                    charge_level: value.charge_level(),
                    accumulated_hit_force: value.accumulated_hit_force(),
                    y_target_dir: value.y_target_dir(),
                    has_damaged: value.has_damaged(),
                    last_damage_tick: value.last_damage_tick(),
                }
            }
        }

        impl<'a, 'b> ::core::cmp::PartialEq<DropshotInfoRef<'a>> for DropshotInfoRef<'b> {
            fn eq(&self, other: &DropshotInfoRef<'_>) -> bool {
                self.charge_level() == other.charge_level()
                    && self.accumulated_hit_force() == other.accumulated_hit_force()
                    && self.y_target_dir() == other.y_target_dir()
                    && self.has_damaged() == other.has_damaged()
                    && self.last_damage_tick() == other.last_damage_tick()
            }
        }

        impl<'a, 'b> ::core::cmp::PartialOrd<DropshotInfoRef<'a>> for DropshotInfoRef<'b> {
            fn partial_cmp(&self, other: &DropshotInfoRef<'_>) -> ::core::option::Option<::core::cmp::Ordering> {
                match self.charge_level().partial_cmp(&other.charge_level()) {
                    ::core::option::Option::Some(::core::cmp::Ordering::Equal) => (),
                    o => return o,
                }

                match self.accumulated_hit_force().partial_cmp(&other.accumulated_hit_force()) {
                    ::core::option::Option::Some(::core::cmp::Ordering::Equal) => (),
                    o => return o,
                }

                match self.y_target_dir().partial_cmp(&other.y_target_dir()) {
                    ::core::option::Option::Some(::core::cmp::Ordering::Equal) => (),
                    o => return o,
                }

                match self.has_damaged().partial_cmp(&other.has_damaged()) {
                    ::core::option::Option::Some(::core::cmp::Ordering::Equal) => (),
                    o => return o,
                }

                self.last_damage_tick().partial_cmp(&other.last_damage_tick())
            }
        }

        impl<'a> ::planus::TableRead<'a> for DropshotInfoRef<'a> {
            #[inline]
            fn from_buffer(
                buffer: ::planus::SliceWithStartOffset<'a>,
                offset: usize,
            ) -> ::core::result::Result<Self, ::planus::errors::ErrorKind> {
                let buffer = buffer.advance_as_array::<24>(offset)?;
                ::core::result::Result::Ok(Self(buffer))
            }
        }

        impl<'a> ::planus::VectorRead<'a> for DropshotInfoRef<'a> {
            const STRIDE: usize = 24;

            #[inline]
            unsafe fn from_buffer(buffer: ::planus::SliceWithStartOffset<'a>, offset: usize) -> Self {
                Self(unsafe { buffer.unchecked_advance_as_array(offset) })
            }
        }

        /// # Safety
        /// The planus compiler generates implementations that initialize
        /// the bytes in `write_values`.
        unsafe impl ::planus::VectorWrite<DropshotInfo> for DropshotInfo {
            const STRIDE: usize = 24;

            type Value = DropshotInfo;

            #[inline]
            fn prepare(&self, _builder: &mut ::planus::Builder) -> Self::Value {
                *self
            }

            #[inline]
            unsafe fn write_values(values: &[DropshotInfo], bytes: *mut ::core::mem::MaybeUninit<u8>, buffer_position: u32) {
                let bytes = bytes as *mut [::core::mem::MaybeUninit<u8>; 24];
                for (i, v) in ::core::iter::Iterator::enumerate(values.iter()) {
                    ::planus::WriteAsPrimitive::write(
                        v,
                        ::planus::Cursor::new(unsafe { &mut *bytes.add(i) }),
                        buffer_position - (24 * i) as u32,
                    );
                }
            }
        }

        /// The struct `BallState` in the namespace `rocketsim`
        ///
        /// Generated from these locations:
        /// * Struct `BallState` in the file `spec/game_state.fbs:92`
        #[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Default, ::serde::Serialize, ::serde::Deserialize)]
        pub struct BallState {
            /// The field `physics` in the struct `BallState`
            pub physics: self::PhysState,

            /// The field `hs_info` in the struct `BallState`
            pub hs_info: self::HeatseekerInfo,

            /// The field `ds_info` in the struct `BallState`
            pub ds_info: self::DropshotInfo,
        }

        /// # Safety
        /// The Planus compiler correctly calculates `ALIGNMENT` and `SIZE`.
        unsafe impl ::planus::Primitive for BallState {
            const ALIGNMENT: usize = 8;
            const SIZE: usize = 112;
        }

        #[allow(clippy::identity_op)]
        impl ::planus::WriteAsPrimitive<BallState> for BallState {
            #[inline]
            fn write<const N: usize>(&self, cursor: ::planus::Cursor<'_, N>, buffer_position: u32) {
                let (cur, cursor) = cursor.split::<72, 40>();
                self.physics.write(cur, buffer_position - 0);
                let (cur, cursor) = cursor.split::<12, 28>();
                self.hs_info.write(cur, buffer_position - 72);
                let cursor = cursor.write::<4, 24>([0; 4]);
                let (cur, cursor) = cursor.split::<24, 0>();
                self.ds_info.write(cur, buffer_position - 88);
                cursor.finish([]);
            }
        }

        impl ::planus::WriteAsOffset<BallState> for BallState {
            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<BallState> {
                unsafe {
                    builder.write_with(112, 7, |buffer_position, bytes| {
                        let bytes = bytes.as_mut_ptr();

                        ::planus::WriteAsPrimitive::write(
                            self,
                            ::planus::Cursor::new(&mut *(bytes as *mut [::core::mem::MaybeUninit<u8>; 112])),
                            buffer_position,
                        );
                    });
                }
                builder.current_offset()
            }
        }

        impl ::planus::WriteAs<BallState> for BallState {
            type Prepared = Self;
            #[inline]
            fn prepare(&self, _builder: &mut ::planus::Builder) -> Self {
                *self
            }
        }

        impl ::planus::WriteAsOptional<BallState> for BallState {
            type Prepared = Self;
            #[inline]
            fn prepare(&self, _builder: &mut ::planus::Builder) -> ::core::option::Option<Self> {
                ::core::option::Option::Some(*self)
            }
        }

        /// Reference to a deserialized [BallState].
        #[derive(Copy, Clone)]
        pub struct BallStateRef<'a>(::planus::ArrayWithStartOffset<'a, 112>);

        impl<'a> BallStateRef<'a> {
            /// Getter for the [`physics` field](BallState#structfield.physics).
            pub fn physics(&self) -> self::PhysStateRef<'a> {
                let buffer = self.0.advance_as_array::<72>(0).unwrap();

                ::core::convert::From::from(buffer)
            }

            /// Getter for the [`hs_info` field](BallState#structfield.hs_info).
            pub fn hs_info(&self) -> self::HeatseekerInfoRef<'a> {
                let buffer = self.0.advance_as_array::<12>(72).unwrap();

                ::core::convert::From::from(buffer)
            }

            /// Getter for the [`ds_info` field](BallState#structfield.ds_info).
            pub fn ds_info(&self) -> self::DropshotInfoRef<'a> {
                let buffer = self.0.advance_as_array::<24>(88).unwrap();

                ::core::convert::From::from(buffer)
            }
        }

        impl<'a> ::core::fmt::Debug for BallStateRef<'a> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                let mut f = f.debug_struct("BallStateRef");
                f.field("physics", &self.physics());
                f.field("hs_info", &self.hs_info());
                f.field("ds_info", &self.ds_info());
                f.finish()
            }
        }

        impl<'a> ::core::convert::From<::planus::ArrayWithStartOffset<'a, 112>> for BallStateRef<'a> {
            fn from(array: ::planus::ArrayWithStartOffset<'a, 112>) -> Self {
                Self(array)
            }
        }

        impl<'a> ::core::convert::From<BallStateRef<'a>> for BallState {
            #[allow(unreachable_code)]
            fn from(value: BallStateRef<'a>) -> Self {
                Self {
                    physics: ::core::convert::From::from(value.physics()),
                    hs_info: ::core::convert::From::from(value.hs_info()),
                    ds_info: ::core::convert::From::from(value.ds_info()),
                }
            }
        }

        impl<'a, 'b> ::core::cmp::PartialEq<BallStateRef<'a>> for BallStateRef<'b> {
            fn eq(&self, other: &BallStateRef<'_>) -> bool {
                self.physics() == other.physics() && self.hs_info() == other.hs_info() && self.ds_info() == other.ds_info()
            }
        }

        impl<'a, 'b> ::core::cmp::PartialOrd<BallStateRef<'a>> for BallStateRef<'b> {
            fn partial_cmp(&self, other: &BallStateRef<'_>) -> ::core::option::Option<::core::cmp::Ordering> {
                match self.physics().partial_cmp(&other.physics()) {
                    ::core::option::Option::Some(::core::cmp::Ordering::Equal) => (),
                    o => return o,
                }

                match self.hs_info().partial_cmp(&other.hs_info()) {
                    ::core::option::Option::Some(::core::cmp::Ordering::Equal) => (),
                    o => return o,
                }

                self.ds_info().partial_cmp(&other.ds_info())
            }
        }

        impl<'a> ::planus::TableRead<'a> for BallStateRef<'a> {
            #[inline]
            fn from_buffer(
                buffer: ::planus::SliceWithStartOffset<'a>,
                offset: usize,
            ) -> ::core::result::Result<Self, ::planus::errors::ErrorKind> {
                let buffer = buffer.advance_as_array::<112>(offset)?;
                ::core::result::Result::Ok(Self(buffer))
            }
        }

        impl<'a> ::planus::VectorRead<'a> for BallStateRef<'a> {
            const STRIDE: usize = 112;

            #[inline]
            unsafe fn from_buffer(buffer: ::planus::SliceWithStartOffset<'a>, offset: usize) -> Self {
                Self(unsafe { buffer.unchecked_advance_as_array(offset) })
            }
        }

        /// # Safety
        /// The planus compiler generates implementations that initialize
        /// the bytes in `write_values`.
        unsafe impl ::planus::VectorWrite<BallState> for BallState {
            const STRIDE: usize = 112;

            type Value = BallState;

            #[inline]
            fn prepare(&self, _builder: &mut ::planus::Builder) -> Self::Value {
                *self
            }

            #[inline]
            unsafe fn write_values(values: &[BallState], bytes: *mut ::core::mem::MaybeUninit<u8>, buffer_position: u32) {
                let bytes = bytes as *mut [::core::mem::MaybeUninit<u8>; 112];
                for (i, v) in ::core::iter::Iterator::enumerate(values.iter()) {
                    ::planus::WriteAsPrimitive::write(
                        v,
                        ::planus::Cursor::new(unsafe { &mut *bytes.add(i) }),
                        buffer_position - (112 * i) as u32,
                    );
                }
            }
        }

        /// The struct `BoostPadConfig` in the namespace `rocketsim`
        ///
        /// Generated from these locations:
        /// * Struct `BoostPadConfig` in the file `spec/game_state.fbs:98`
        #[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Default, ::serde::Serialize, ::serde::Deserialize)]
        pub struct BoostPadConfig {
            /// The field `pos` in the struct `BoostPadConfig`
            pub pos: self::Vec3,

            /// The field `is_big` in the struct `BoostPadConfig`
            pub is_big: bool,
        }

        /// # Safety
        /// The Planus compiler correctly calculates `ALIGNMENT` and `SIZE`.
        unsafe impl ::planus::Primitive for BoostPadConfig {
            const ALIGNMENT: usize = 4;
            const SIZE: usize = 16;
        }

        #[allow(clippy::identity_op)]
        impl ::planus::WriteAsPrimitive<BoostPadConfig> for BoostPadConfig {
            #[inline]
            fn write<const N: usize>(&self, cursor: ::planus::Cursor<'_, N>, buffer_position: u32) {
                let (cur, cursor) = cursor.split::<12, 4>();
                self.pos.write(cur, buffer_position - 0);
                let (cur, cursor) = cursor.split::<1, 3>();
                self.is_big.write(cur, buffer_position - 12);
                let cursor = cursor.write::<3, 0>([0; 3]);
                cursor.finish([]);
            }
        }

        impl ::planus::WriteAsOffset<BoostPadConfig> for BoostPadConfig {
            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<BoostPadConfig> {
                unsafe {
                    builder.write_with(16, 3, |buffer_position, bytes| {
                        let bytes = bytes.as_mut_ptr();

                        ::planus::WriteAsPrimitive::write(
                            self,
                            ::planus::Cursor::new(&mut *(bytes as *mut [::core::mem::MaybeUninit<u8>; 16])),
                            buffer_position,
                        );
                    });
                }
                builder.current_offset()
            }
        }

        impl ::planus::WriteAs<BoostPadConfig> for BoostPadConfig {
            type Prepared = Self;
            #[inline]
            fn prepare(&self, _builder: &mut ::planus::Builder) -> Self {
                *self
            }
        }

        impl ::planus::WriteAsOptional<BoostPadConfig> for BoostPadConfig {
            type Prepared = Self;
            #[inline]
            fn prepare(&self, _builder: &mut ::planus::Builder) -> ::core::option::Option<Self> {
                ::core::option::Option::Some(*self)
            }
        }

        /// Reference to a deserialized [BoostPadConfig].
        #[derive(Copy, Clone)]
        pub struct BoostPadConfigRef<'a>(::planus::ArrayWithStartOffset<'a, 16>);

        impl<'a> BoostPadConfigRef<'a> {
            /// Getter for the [`pos` field](BoostPadConfig#structfield.pos).
            pub fn pos(&self) -> self::Vec3Ref<'a> {
                let buffer = self.0.advance_as_array::<12>(0).unwrap();

                ::core::convert::From::from(buffer)
            }

            /// Getter for the [`is_big` field](BoostPadConfig#structfield.is_big).
            pub fn is_big(&self) -> bool {
                let buffer = self.0.advance_as_array::<1>(12).unwrap();

                buffer.as_array()[0] != 0
            }
        }

        impl<'a> ::core::fmt::Debug for BoostPadConfigRef<'a> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                let mut f = f.debug_struct("BoostPadConfigRef");
                f.field("pos", &self.pos());
                f.field("is_big", &self.is_big());
                f.finish()
            }
        }

        impl<'a> ::core::convert::From<::planus::ArrayWithStartOffset<'a, 16>> for BoostPadConfigRef<'a> {
            fn from(array: ::planus::ArrayWithStartOffset<'a, 16>) -> Self {
                Self(array)
            }
        }

        impl<'a> ::core::convert::From<BoostPadConfigRef<'a>> for BoostPadConfig {
            #[allow(unreachable_code)]
            fn from(value: BoostPadConfigRef<'a>) -> Self {
                Self {
                    pos: ::core::convert::From::from(value.pos()),
                    is_big: value.is_big(),
                }
            }
        }

        impl<'a, 'b> ::core::cmp::PartialEq<BoostPadConfigRef<'a>> for BoostPadConfigRef<'b> {
            fn eq(&self, other: &BoostPadConfigRef<'_>) -> bool {
                self.pos() == other.pos() && self.is_big() == other.is_big()
            }
        }

        impl<'a, 'b> ::core::cmp::PartialOrd<BoostPadConfigRef<'a>> for BoostPadConfigRef<'b> {
            fn partial_cmp(&self, other: &BoostPadConfigRef<'_>) -> ::core::option::Option<::core::cmp::Ordering> {
                match self.pos().partial_cmp(&other.pos()) {
                    ::core::option::Option::Some(::core::cmp::Ordering::Equal) => (),
                    o => return o,
                }

                self.is_big().partial_cmp(&other.is_big())
            }
        }

        impl<'a> ::planus::TableRead<'a> for BoostPadConfigRef<'a> {
            #[inline]
            fn from_buffer(
                buffer: ::planus::SliceWithStartOffset<'a>,
                offset: usize,
            ) -> ::core::result::Result<Self, ::planus::errors::ErrorKind> {
                let buffer = buffer.advance_as_array::<16>(offset)?;
                ::core::result::Result::Ok(Self(buffer))
            }
        }

        impl<'a> ::planus::VectorRead<'a> for BoostPadConfigRef<'a> {
            const STRIDE: usize = 16;

            #[inline]
            unsafe fn from_buffer(buffer: ::planus::SliceWithStartOffset<'a>, offset: usize) -> Self {
                Self(unsafe { buffer.unchecked_advance_as_array(offset) })
            }
        }

        /// # Safety
        /// The planus compiler generates implementations that initialize
        /// the bytes in `write_values`.
        unsafe impl ::planus::VectorWrite<BoostPadConfig> for BoostPadConfig {
            const STRIDE: usize = 16;

            type Value = BoostPadConfig;

            #[inline]
            fn prepare(&self, _builder: &mut ::planus::Builder) -> Self::Value {
                *self
            }

            #[inline]
            unsafe fn write_values(
                values: &[BoostPadConfig],
                bytes: *mut ::core::mem::MaybeUninit<u8>,
                buffer_position: u32,
            ) {
                let bytes = bytes as *mut [::core::mem::MaybeUninit<u8>; 16];
                for (i, v) in ::core::iter::Iterator::enumerate(values.iter()) {
                    ::planus::WriteAsPrimitive::write(
                        v,
                        ::planus::Cursor::new(unsafe { &mut *bytes.add(i) }),
                        buffer_position - (16 * i) as u32,
                    );
                }
            }
        }

        /// The struct `BoostPadState` in the namespace `rocketsim`
        ///
        /// Generated from these locations:
        /// * Struct `BoostPadState` in the file `spec/game_state.fbs:103`
        #[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Default, ::serde::Serialize, ::serde::Deserialize)]
        pub struct BoostPadState {
            /// The field `is_active` in the struct `BoostPadState`
            pub is_active: bool,

            /// The field `cooldown` in the struct `BoostPadState`
            pub cooldown: f32,

            /// The field `cur_locked_car` in the struct `BoostPadState`
            pub cur_locked_car: u64,

            /// The field `prev_locked_car_id` in the struct `BoostPadState`
            pub prev_locked_car_id: u64,
        }

        /// # Safety
        /// The Planus compiler correctly calculates `ALIGNMENT` and `SIZE`.
        unsafe impl ::planus::Primitive for BoostPadState {
            const ALIGNMENT: usize = 8;
            const SIZE: usize = 24;
        }

        #[allow(clippy::identity_op)]
        impl ::planus::WriteAsPrimitive<BoostPadState> for BoostPadState {
            #[inline]
            fn write<const N: usize>(&self, cursor: ::planus::Cursor<'_, N>, buffer_position: u32) {
                let (cur, cursor) = cursor.split::<1, 23>();
                self.is_active.write(cur, buffer_position - 0);
                let cursor = cursor.write::<3, 20>([0; 3]);
                let (cur, cursor) = cursor.split::<4, 16>();
                self.cooldown.write(cur, buffer_position - 4);
                let (cur, cursor) = cursor.split::<8, 8>();
                self.cur_locked_car.write(cur, buffer_position - 8);
                let (cur, cursor) = cursor.split::<8, 0>();
                self.prev_locked_car_id.write(cur, buffer_position - 16);
                cursor.finish([]);
            }
        }

        impl ::planus::WriteAsOffset<BoostPadState> for BoostPadState {
            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<BoostPadState> {
                unsafe {
                    builder.write_with(24, 7, |buffer_position, bytes| {
                        let bytes = bytes.as_mut_ptr();

                        ::planus::WriteAsPrimitive::write(
                            self,
                            ::planus::Cursor::new(&mut *(bytes as *mut [::core::mem::MaybeUninit<u8>; 24])),
                            buffer_position,
                        );
                    });
                }
                builder.current_offset()
            }
        }

        impl ::planus::WriteAs<BoostPadState> for BoostPadState {
            type Prepared = Self;
            #[inline]
            fn prepare(&self, _builder: &mut ::planus::Builder) -> Self {
                *self
            }
        }

        impl ::planus::WriteAsOptional<BoostPadState> for BoostPadState {
            type Prepared = Self;
            #[inline]
            fn prepare(&self, _builder: &mut ::planus::Builder) -> ::core::option::Option<Self> {
                ::core::option::Option::Some(*self)
            }
        }

        /// Reference to a deserialized [BoostPadState].
        #[derive(Copy, Clone)]
        pub struct BoostPadStateRef<'a>(::planus::ArrayWithStartOffset<'a, 24>);

        impl<'a> BoostPadStateRef<'a> {
            /// Getter for the [`is_active` field](BoostPadState#structfield.is_active).
            pub fn is_active(&self) -> bool {
                let buffer = self.0.advance_as_array::<1>(0).unwrap();

                buffer.as_array()[0] != 0
            }

            /// Getter for the [`cooldown` field](BoostPadState#structfield.cooldown).
            pub fn cooldown(&self) -> f32 {
                let buffer = self.0.advance_as_array::<4>(4).unwrap();

                f32::from_le_bytes(*buffer.as_array())
            }

            /// Getter for the [`cur_locked_car` field](BoostPadState#structfield.cur_locked_car).
            pub fn cur_locked_car(&self) -> u64 {
                let buffer = self.0.advance_as_array::<8>(8).unwrap();

                u64::from_le_bytes(*buffer.as_array())
            }

            /// Getter for the [`prev_locked_car_id` field](BoostPadState#structfield.prev_locked_car_id).
            pub fn prev_locked_car_id(&self) -> u64 {
                let buffer = self.0.advance_as_array::<8>(16).unwrap();

                u64::from_le_bytes(*buffer.as_array())
            }
        }

        impl<'a> ::core::fmt::Debug for BoostPadStateRef<'a> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                let mut f = f.debug_struct("BoostPadStateRef");
                f.field("is_active", &self.is_active());
                f.field("cooldown", &self.cooldown());
                f.field("cur_locked_car", &self.cur_locked_car());
                f.field("prev_locked_car_id", &self.prev_locked_car_id());
                f.finish()
            }
        }

        impl<'a> ::core::convert::From<::planus::ArrayWithStartOffset<'a, 24>> for BoostPadStateRef<'a> {
            fn from(array: ::planus::ArrayWithStartOffset<'a, 24>) -> Self {
                Self(array)
            }
        }

        impl<'a> ::core::convert::From<BoostPadStateRef<'a>> for BoostPadState {
            #[allow(unreachable_code)]
            fn from(value: BoostPadStateRef<'a>) -> Self {
                Self {
                    is_active: value.is_active(),
                    cooldown: value.cooldown(),
                    cur_locked_car: value.cur_locked_car(),
                    prev_locked_car_id: value.prev_locked_car_id(),
                }
            }
        }

        impl<'a, 'b> ::core::cmp::PartialEq<BoostPadStateRef<'a>> for BoostPadStateRef<'b> {
            fn eq(&self, other: &BoostPadStateRef<'_>) -> bool {
                self.is_active() == other.is_active()
                    && self.cooldown() == other.cooldown()
                    && self.cur_locked_car() == other.cur_locked_car()
                    && self.prev_locked_car_id() == other.prev_locked_car_id()
            }
        }

        impl<'a, 'b> ::core::cmp::PartialOrd<BoostPadStateRef<'a>> for BoostPadStateRef<'b> {
            fn partial_cmp(&self, other: &BoostPadStateRef<'_>) -> ::core::option::Option<::core::cmp::Ordering> {
                match self.is_active().partial_cmp(&other.is_active()) {
                    ::core::option::Option::Some(::core::cmp::Ordering::Equal) => (),
                    o => return o,
                }

                match self.cooldown().partial_cmp(&other.cooldown()) {
                    ::core::option::Option::Some(::core::cmp::Ordering::Equal) => (),
                    o => return o,
                }

                match self.cur_locked_car().partial_cmp(&other.cur_locked_car()) {
                    ::core::option::Option::Some(::core::cmp::Ordering::Equal) => (),
                    o => return o,
                }

                self.prev_locked_car_id().partial_cmp(&other.prev_locked_car_id())
            }
        }

        impl<'a> ::planus::TableRead<'a> for BoostPadStateRef<'a> {
            #[inline]
            fn from_buffer(
                buffer: ::planus::SliceWithStartOffset<'a>,
                offset: usize,
            ) -> ::core::result::Result<Self, ::planus::errors::ErrorKind> {
                let buffer = buffer.advance_as_array::<24>(offset)?;
                ::core::result::Result::Ok(Self(buffer))
            }
        }

        impl<'a> ::planus::VectorRead<'a> for BoostPadStateRef<'a> {
            const STRIDE: usize = 24;

            #[inline]
            unsafe fn from_buffer(buffer: ::planus::SliceWithStartOffset<'a>, offset: usize) -> Self {
                Self(unsafe { buffer.unchecked_advance_as_array(offset) })
            }
        }

        /// # Safety
        /// The planus compiler generates implementations that initialize
        /// the bytes in `write_values`.
        unsafe impl ::planus::VectorWrite<BoostPadState> for BoostPadState {
            const STRIDE: usize = 24;

            type Value = BoostPadState;

            #[inline]
            fn prepare(&self, _builder: &mut ::planus::Builder) -> Self::Value {
                *self
            }

            #[inline]
            unsafe fn write_values(
                values: &[BoostPadState],
                bytes: *mut ::core::mem::MaybeUninit<u8>,
                buffer_position: u32,
            ) {
                let bytes = bytes as *mut [::core::mem::MaybeUninit<u8>; 24];
                for (i, v) in ::core::iter::Iterator::enumerate(values.iter()) {
                    ::planus::WriteAsPrimitive::write(
                        v,
                        ::planus::Cursor::new(unsafe { &mut *bytes.add(i) }),
                        buffer_position - (24 * i) as u32,
                    );
                }
            }
        }

        /// The table `BoostPadInfo` in the namespace `rocketsim`
        ///
        /// Generated from these locations:
        /// * Table `BoostPadInfo` in the file `spec/game_state.fbs:110`
        #[derive(Clone, Debug, PartialEq, PartialOrd, ::serde::Serialize, ::serde::Deserialize)]
        pub struct BoostPadInfo {
            /// The field `config` in the table `BoostPadInfo`
            pub config: self::BoostPadConfig,
            /// The field `state` in the table `BoostPadInfo`
            pub state: self::BoostPadState,
        }

        #[allow(clippy::derivable_impls)]
        impl ::core::default::Default for BoostPadInfo {
            fn default() -> Self {
                Self {
                    config: ::core::default::Default::default(),
                    state: ::core::default::Default::default(),
                }
            }
        }

        impl BoostPadInfo {
            /// Creates a [BoostPadInfoBuilder] for serializing an instance of this table.
            #[inline]
            pub fn builder() -> BoostPadInfoBuilder<()> {
                BoostPadInfoBuilder(())
            }

            #[allow(clippy::too_many_arguments)]
            pub fn create(
                builder: &mut ::planus::Builder,
                field_config: impl ::planus::WriteAs<self::BoostPadConfig>,
                field_state: impl ::planus::WriteAs<self::BoostPadState>,
            ) -> ::planus::Offset<Self> {
                let prepared_config = field_config.prepare(builder);
                let prepared_state = field_state.prepare(builder);

                let mut table_writer: ::planus::table_writer::TableWriter<8> = ::core::default::Default::default();
                table_writer.write_entry::<self::BoostPadState>(1);
                table_writer.write_entry::<self::BoostPadConfig>(0);

                unsafe {
                    table_writer.finish(builder, |object_writer| {
                        object_writer.write::<_, _, 24>(&prepared_state);
                        object_writer.write::<_, _, 16>(&prepared_config);
                    });
                }
                builder.current_offset()
            }
        }

        impl ::planus::WriteAs<::planus::Offset<BoostPadInfo>> for BoostPadInfo {
            type Prepared = ::planus::Offset<Self>;

            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<BoostPadInfo> {
                ::planus::WriteAsOffset::prepare(self, builder)
            }
        }

        impl ::planus::WriteAsOptional<::planus::Offset<BoostPadInfo>> for BoostPadInfo {
            type Prepared = ::planus::Offset<Self>;

            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::core::option::Option<::planus::Offset<BoostPadInfo>> {
                ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
            }
        }

        impl ::planus::WriteAsOffset<BoostPadInfo> for BoostPadInfo {
            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<BoostPadInfo> {
                BoostPadInfo::create(builder, self.config, self.state)
            }
        }

        /// Builder for serializing an instance of the [BoostPadInfo] type.
        ///
        /// Can be created using the [BoostPadInfo::builder] method.
        #[derive(Debug)]
        #[must_use]
        pub struct BoostPadInfoBuilder<State>(State);

        impl BoostPadInfoBuilder<()> {
            /// Setter for the [`config` field](BoostPadInfo#structfield.config).
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn config<T0>(self, value: T0) -> BoostPadInfoBuilder<(T0,)>
            where
                T0: ::planus::WriteAs<self::BoostPadConfig>,
            {
                BoostPadInfoBuilder((value,))
            }
        }

        impl<T0> BoostPadInfoBuilder<(T0,)> {
            /// Setter for the [`state` field](BoostPadInfo#structfield.state).
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn state<T1>(self, value: T1) -> BoostPadInfoBuilder<(T0, T1)>
            where
                T1: ::planus::WriteAs<self::BoostPadState>,
            {
                let (v0,) = self.0;
                BoostPadInfoBuilder((v0, value))
            }
        }

        impl<T0, T1> BoostPadInfoBuilder<(T0, T1)> {
            /// Finish writing the builder to get an [Offset](::planus::Offset) to a serialized [BoostPadInfo].
            #[inline]
            pub fn finish(self, builder: &mut ::planus::Builder) -> ::planus::Offset<BoostPadInfo>
            where
                Self: ::planus::WriteAsOffset<BoostPadInfo>,
            {
                ::planus::WriteAsOffset::prepare(&self, builder)
            }
        }

        impl<T0: ::planus::WriteAs<self::BoostPadConfig>, T1: ::planus::WriteAs<self::BoostPadState>>
            ::planus::WriteAs<::planus::Offset<BoostPadInfo>> for BoostPadInfoBuilder<(T0, T1)>
        {
            type Prepared = ::planus::Offset<BoostPadInfo>;

            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<BoostPadInfo> {
                ::planus::WriteAsOffset::prepare(self, builder)
            }
        }

        impl<T0: ::planus::WriteAs<self::BoostPadConfig>, T1: ::planus::WriteAs<self::BoostPadState>>
            ::planus::WriteAsOptional<::planus::Offset<BoostPadInfo>> for BoostPadInfoBuilder<(T0, T1)>
        {
            type Prepared = ::planus::Offset<BoostPadInfo>;

            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::core::option::Option<::planus::Offset<BoostPadInfo>> {
                ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
            }
        }

        impl<T0: ::planus::WriteAs<self::BoostPadConfig>, T1: ::planus::WriteAs<self::BoostPadState>>
            ::planus::WriteAsOffset<BoostPadInfo> for BoostPadInfoBuilder<(T0, T1)>
        {
            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<BoostPadInfo> {
                let (v0, v1) = &self.0;
                BoostPadInfo::create(builder, v0, v1)
            }
        }

        /// Reference to a deserialized [BoostPadInfo].
        #[derive(Copy, Clone)]
        pub struct BoostPadInfoRef<'a>(#[allow(dead_code)] ::planus::table_reader::Table<'a>);

        impl<'a> BoostPadInfoRef<'a> {
            /// Getter for the [`config` field](BoostPadInfo#structfield.config).
            #[inline]
            pub fn config(&self) -> ::planus::Result<self::BoostPadConfigRef<'a>> {
                self.0.access_required(0, "BoostPadInfo", "config")
            }

            /// Getter for the [`state` field](BoostPadInfo#structfield.state).
            #[inline]
            pub fn state(&self) -> ::planus::Result<self::BoostPadStateRef<'a>> {
                self.0.access_required(1, "BoostPadInfo", "state")
            }
        }

        impl<'a> ::core::fmt::Debug for BoostPadInfoRef<'a> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                let mut f = f.debug_struct("BoostPadInfoRef");
                f.field("config", &self.config());
                f.field("state", &self.state());
                f.finish()
            }
        }

        impl<'a> ::core::convert::TryFrom<BoostPadInfoRef<'a>> for BoostPadInfo {
            type Error = ::planus::Error;

            #[allow(unreachable_code)]
            fn try_from(value: BoostPadInfoRef<'a>) -> ::planus::Result<Self> {
                ::core::result::Result::Ok(Self {
                    config: ::core::convert::Into::into(value.config()?),
                    state: ::core::convert::Into::into(value.state()?),
                })
            }
        }

        impl<'a> ::planus::TableRead<'a> for BoostPadInfoRef<'a> {
            #[inline]
            fn from_buffer(
                buffer: ::planus::SliceWithStartOffset<'a>,
                offset: usize,
            ) -> ::core::result::Result<Self, ::planus::errors::ErrorKind> {
                ::core::result::Result::Ok(Self(::planus::table_reader::Table::from_buffer(buffer, offset)?))
            }
        }

        impl<'a> ::planus::VectorReadInner<'a> for BoostPadInfoRef<'a> {
            type Error = ::planus::Error;
            const STRIDE: usize = 4;

            unsafe fn from_buffer(buffer: ::planus::SliceWithStartOffset<'a>, offset: usize) -> ::planus::Result<Self> {
                ::planus::TableRead::from_buffer(buffer, offset).map_err(|error_kind| {
                    error_kind.with_error_location("[BoostPadInfoRef]", "get", buffer.offset_from_start)
                })
            }
        }

        /// # Safety
        /// The planus compiler generates implementations that initialize
        /// the bytes in `write_values`.
        unsafe impl ::planus::VectorWrite<::planus::Offset<BoostPadInfo>> for BoostPadInfo {
            type Value = ::planus::Offset<BoostPadInfo>;
            const STRIDE: usize = 4;
            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> Self::Value {
                ::planus::WriteAs::prepare(self, builder)
            }

            #[inline]
            unsafe fn write_values(
                values: &[::planus::Offset<BoostPadInfo>],
                bytes: *mut ::core::mem::MaybeUninit<u8>,
                buffer_position: u32,
            ) {
                let bytes = bytes as *mut [::core::mem::MaybeUninit<u8>; 4];
                for (i, v) in ::core::iter::Iterator::enumerate(values.iter()) {
                    ::planus::WriteAsPrimitive::write(
                        v,
                        ::planus::Cursor::new(unsafe { &mut *bytes.add(i) }),
                        buffer_position - (Self::STRIDE * i) as u32,
                    );
                }
            }
        }

        impl<'a> ::planus::ReadAsRoot<'a> for BoostPadInfoRef<'a> {
            fn read_as_root(slice: &'a [u8]) -> ::planus::Result<Self> {
                ::planus::TableRead::from_buffer(
                    ::planus::SliceWithStartOffset {
                        buffer: slice,
                        offset_from_start: 0,
                    },
                    0,
                )
                .map_err(|error_kind| error_kind.with_error_location("[BoostPadInfoRef]", "read_as_root", 0))
            }
        }

        /// The table `DropshotTile` in the namespace `rocketsim`
        ///
        /// Generated from these locations:
        /// * Table `DropshotTile` in the file `spec/game_state.fbs:115`
        #[derive(Clone, Debug, PartialEq, PartialOrd, ::serde::Serialize, ::serde::Deserialize)]
        pub struct DropshotTile {
            /// The field `pos` in the table `DropshotTile`
            pub pos: self::Vec3,
            /// The field `state` in the table `DropshotTile`
            pub state: self::TileState,
        }

        #[allow(clippy::derivable_impls)]
        impl ::core::default::Default for DropshotTile {
            fn default() -> Self {
                Self {
                    pos: ::core::default::Default::default(),
                    state: self::TileState::Full,
                }
            }
        }

        impl DropshotTile {
            /// Creates a [DropshotTileBuilder] for serializing an instance of this table.
            #[inline]
            pub fn builder() -> DropshotTileBuilder<()> {
                DropshotTileBuilder(())
            }

            #[allow(clippy::too_many_arguments)]
            pub fn create(
                builder: &mut ::planus::Builder,
                field_pos: impl ::planus::WriteAs<self::Vec3>,
                field_state: impl ::planus::WriteAsDefault<self::TileState, self::TileState>,
            ) -> ::planus::Offset<Self> {
                let prepared_pos = field_pos.prepare(builder);
                let prepared_state = field_state.prepare(builder, &self::TileState::Full);

                let mut table_writer: ::planus::table_writer::TableWriter<8> = ::core::default::Default::default();
                table_writer.write_entry::<self::Vec3>(0);
                if prepared_state.is_some() {
                    table_writer.write_entry::<self::TileState>(1);
                }

                unsafe {
                    table_writer.finish(builder, |object_writer| {
                        object_writer.write::<_, _, 12>(&prepared_pos);
                        if let ::core::option::Option::Some(prepared_state) = prepared_state {
                            object_writer.write::<_, _, 1>(&prepared_state);
                        }
                    });
                }
                builder.current_offset()
            }
        }

        impl ::planus::WriteAs<::planus::Offset<DropshotTile>> for DropshotTile {
            type Prepared = ::planus::Offset<Self>;

            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<DropshotTile> {
                ::planus::WriteAsOffset::prepare(self, builder)
            }
        }

        impl ::planus::WriteAsOptional<::planus::Offset<DropshotTile>> for DropshotTile {
            type Prepared = ::planus::Offset<Self>;

            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::core::option::Option<::planus::Offset<DropshotTile>> {
                ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
            }
        }

        impl ::planus::WriteAsOffset<DropshotTile> for DropshotTile {
            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<DropshotTile> {
                DropshotTile::create(builder, self.pos, self.state)
            }
        }

        /// Builder for serializing an instance of the [DropshotTile] type.
        ///
        /// Can be created using the [DropshotTile::builder] method.
        #[derive(Debug)]
        #[must_use]
        pub struct DropshotTileBuilder<State>(State);

        impl DropshotTileBuilder<()> {
            /// Setter for the [`pos` field](DropshotTile#structfield.pos).
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn pos<T0>(self, value: T0) -> DropshotTileBuilder<(T0,)>
            where
                T0: ::planus::WriteAs<self::Vec3>,
            {
                DropshotTileBuilder((value,))
            }
        }

        impl<T0> DropshotTileBuilder<(T0,)> {
            /// Setter for the [`state` field](DropshotTile#structfield.state).
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn state<T1>(self, value: T1) -> DropshotTileBuilder<(T0, T1)>
            where
                T1: ::planus::WriteAsDefault<self::TileState, self::TileState>,
            {
                let (v0,) = self.0;
                DropshotTileBuilder((v0, value))
            }

            /// Sets the [`state` field](DropshotTile#structfield.state) to the default value.
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn state_as_default(self) -> DropshotTileBuilder<(T0, ::planus::DefaultValue)> {
                self.state(::planus::DefaultValue)
            }
        }

        impl<T0, T1> DropshotTileBuilder<(T0, T1)> {
            /// Finish writing the builder to get an [Offset](::planus::Offset) to a serialized [DropshotTile].
            #[inline]
            pub fn finish(self, builder: &mut ::planus::Builder) -> ::planus::Offset<DropshotTile>
            where
                Self: ::planus::WriteAsOffset<DropshotTile>,
            {
                ::planus::WriteAsOffset::prepare(&self, builder)
            }
        }

        impl<T0: ::planus::WriteAs<self::Vec3>, T1: ::planus::WriteAsDefault<self::TileState, self::TileState>>
            ::planus::WriteAs<::planus::Offset<DropshotTile>> for DropshotTileBuilder<(T0, T1)>
        {
            type Prepared = ::planus::Offset<DropshotTile>;

            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<DropshotTile> {
                ::planus::WriteAsOffset::prepare(self, builder)
            }
        }

        impl<T0: ::planus::WriteAs<self::Vec3>, T1: ::planus::WriteAsDefault<self::TileState, self::TileState>>
            ::planus::WriteAsOptional<::planus::Offset<DropshotTile>> for DropshotTileBuilder<(T0, T1)>
        {
            type Prepared = ::planus::Offset<DropshotTile>;

            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::core::option::Option<::planus::Offset<DropshotTile>> {
                ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
            }
        }

        impl<T0: ::planus::WriteAs<self::Vec3>, T1: ::planus::WriteAsDefault<self::TileState, self::TileState>>
            ::planus::WriteAsOffset<DropshotTile> for DropshotTileBuilder<(T0, T1)>
        {
            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<DropshotTile> {
                let (v0, v1) = &self.0;
                DropshotTile::create(builder, v0, v1)
            }
        }

        /// Reference to a deserialized [DropshotTile].
        #[derive(Copy, Clone)]
        pub struct DropshotTileRef<'a>(#[allow(dead_code)] ::planus::table_reader::Table<'a>);

        impl<'a> DropshotTileRef<'a> {
            /// Getter for the [`pos` field](DropshotTile#structfield.pos).
            #[inline]
            pub fn pos(&self) -> ::planus::Result<self::Vec3Ref<'a>> {
                self.0.access_required(0, "DropshotTile", "pos")
            }

            /// Getter for the [`state` field](DropshotTile#structfield.state).
            #[inline]
            pub fn state(&self) -> ::planus::Result<self::TileState> {
                ::core::result::Result::Ok(self.0.access(1, "DropshotTile", "state")?.unwrap_or(self::TileState::Full))
            }
        }

        impl<'a> ::core::fmt::Debug for DropshotTileRef<'a> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                let mut f = f.debug_struct("DropshotTileRef");
                f.field("pos", &self.pos());
                f.field("state", &self.state());
                f.finish()
            }
        }

        impl<'a> ::core::convert::TryFrom<DropshotTileRef<'a>> for DropshotTile {
            type Error = ::planus::Error;

            #[allow(unreachable_code)]
            fn try_from(value: DropshotTileRef<'a>) -> ::planus::Result<Self> {
                ::core::result::Result::Ok(Self {
                    pos: ::core::convert::Into::into(value.pos()?),
                    state: ::core::convert::TryInto::try_into(value.state()?)?,
                })
            }
        }

        impl<'a> ::planus::TableRead<'a> for DropshotTileRef<'a> {
            #[inline]
            fn from_buffer(
                buffer: ::planus::SliceWithStartOffset<'a>,
                offset: usize,
            ) -> ::core::result::Result<Self, ::planus::errors::ErrorKind> {
                ::core::result::Result::Ok(Self(::planus::table_reader::Table::from_buffer(buffer, offset)?))
            }
        }

        impl<'a> ::planus::VectorReadInner<'a> for DropshotTileRef<'a> {
            type Error = ::planus::Error;
            const STRIDE: usize = 4;

            unsafe fn from_buffer(buffer: ::planus::SliceWithStartOffset<'a>, offset: usize) -> ::planus::Result<Self> {
                ::planus::TableRead::from_buffer(buffer, offset).map_err(|error_kind| {
                    error_kind.with_error_location("[DropshotTileRef]", "get", buffer.offset_from_start)
                })
            }
        }

        /// # Safety
        /// The planus compiler generates implementations that initialize
        /// the bytes in `write_values`.
        unsafe impl ::planus::VectorWrite<::planus::Offset<DropshotTile>> for DropshotTile {
            type Value = ::planus::Offset<DropshotTile>;
            const STRIDE: usize = 4;
            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> Self::Value {
                ::planus::WriteAs::prepare(self, builder)
            }

            #[inline]
            unsafe fn write_values(
                values: &[::planus::Offset<DropshotTile>],
                bytes: *mut ::core::mem::MaybeUninit<u8>,
                buffer_position: u32,
            ) {
                let bytes = bytes as *mut [::core::mem::MaybeUninit<u8>; 4];
                for (i, v) in ::core::iter::Iterator::enumerate(values.iter()) {
                    ::planus::WriteAsPrimitive::write(
                        v,
                        ::planus::Cursor::new(unsafe { &mut *bytes.add(i) }),
                        buffer_position - (Self::STRIDE * i) as u32,
                    );
                }
            }
        }

        impl<'a> ::planus::ReadAsRoot<'a> for DropshotTileRef<'a> {
            fn read_as_root(slice: &'a [u8]) -> ::planus::Result<Self> {
                ::planus::TableRead::from_buffer(
                    ::planus::SliceWithStartOffset {
                        buffer: slice,
                        offset_from_start: 0,
                    },
                    0,
                )
                .map_err(|error_kind| error_kind.with_error_location("[DropshotTileRef]", "read_as_root", 0))
            }
        }

        ///  Two-sided Dropshot tile layout (`Some([blue_tiles, orange_tiles])`).
        ///
        /// Generated from these locations:
        /// * Table `DropshotTilesByTeam` in the file `spec/game_state.fbs:121`
        #[derive(Clone, Debug, PartialEq, PartialOrd, ::serde::Serialize, ::serde::Deserialize)]
        pub struct DropshotTilesByTeam {
            /// The field `blue_tiles` in the table `DropshotTilesByTeam`
            pub blue_tiles: ::planus::alloc::vec::Vec<self::DropshotTile>,
            /// The field `orange_tiles` in the table `DropshotTilesByTeam`
            pub orange_tiles: ::planus::alloc::vec::Vec<self::DropshotTile>,
        }

        #[allow(clippy::derivable_impls)]
        impl ::core::default::Default for DropshotTilesByTeam {
            fn default() -> Self {
                Self {
                    blue_tiles: ::core::default::Default::default(),
                    orange_tiles: ::core::default::Default::default(),
                }
            }
        }

        impl DropshotTilesByTeam {
            /// Creates a [DropshotTilesByTeamBuilder] for serializing an instance of this table.
            #[inline]
            pub fn builder() -> DropshotTilesByTeamBuilder<()> {
                DropshotTilesByTeamBuilder(())
            }

            #[allow(clippy::too_many_arguments)]
            pub fn create(
                builder: &mut ::planus::Builder,
                field_blue_tiles: impl ::planus::WriteAs<::planus::Offset<[::planus::Offset<self::DropshotTile>]>>,
                field_orange_tiles: impl ::planus::WriteAs<::planus::Offset<[::planus::Offset<self::DropshotTile>]>>,
            ) -> ::planus::Offset<Self> {
                let prepared_blue_tiles = field_blue_tiles.prepare(builder);
                let prepared_orange_tiles = field_orange_tiles.prepare(builder);

                let mut table_writer: ::planus::table_writer::TableWriter<8> = ::core::default::Default::default();
                table_writer.write_entry::<::planus::Offset<[::planus::Offset<self::DropshotTile>]>>(0);
                table_writer.write_entry::<::planus::Offset<[::planus::Offset<self::DropshotTile>]>>(1);

                unsafe {
                    table_writer.finish(builder, |object_writer| {
                        object_writer.write::<_, _, 4>(&prepared_blue_tiles);
                        object_writer.write::<_, _, 4>(&prepared_orange_tiles);
                    });
                }
                builder.current_offset()
            }
        }

        impl ::planus::WriteAs<::planus::Offset<DropshotTilesByTeam>> for DropshotTilesByTeam {
            type Prepared = ::planus::Offset<Self>;

            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<DropshotTilesByTeam> {
                ::planus::WriteAsOffset::prepare(self, builder)
            }
        }

        impl ::planus::WriteAsOptional<::planus::Offset<DropshotTilesByTeam>> for DropshotTilesByTeam {
            type Prepared = ::planus::Offset<Self>;

            #[inline]
            fn prepare(
                &self,
                builder: &mut ::planus::Builder,
            ) -> ::core::option::Option<::planus::Offset<DropshotTilesByTeam>> {
                ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
            }
        }

        impl ::planus::WriteAsOffset<DropshotTilesByTeam> for DropshotTilesByTeam {
            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<DropshotTilesByTeam> {
                DropshotTilesByTeam::create(builder, &self.blue_tiles, &self.orange_tiles)
            }
        }

        /// Builder for serializing an instance of the [DropshotTilesByTeam] type.
        ///
        /// Can be created using the [DropshotTilesByTeam::builder] method.
        #[derive(Debug)]
        #[must_use]
        pub struct DropshotTilesByTeamBuilder<State>(State);

        impl DropshotTilesByTeamBuilder<()> {
            /// Setter for the [`blue_tiles` field](DropshotTilesByTeam#structfield.blue_tiles).
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn blue_tiles<T0>(self, value: T0) -> DropshotTilesByTeamBuilder<(T0,)>
            where
                T0: ::planus::WriteAs<::planus::Offset<[::planus::Offset<self::DropshotTile>]>>,
            {
                DropshotTilesByTeamBuilder((value,))
            }
        }

        impl<T0> DropshotTilesByTeamBuilder<(T0,)> {
            /// Setter for the [`orange_tiles` field](DropshotTilesByTeam#structfield.orange_tiles).
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn orange_tiles<T1>(self, value: T1) -> DropshotTilesByTeamBuilder<(T0, T1)>
            where
                T1: ::planus::WriteAs<::planus::Offset<[::planus::Offset<self::DropshotTile>]>>,
            {
                let (v0,) = self.0;
                DropshotTilesByTeamBuilder((v0, value))
            }
        }

        impl<T0, T1> DropshotTilesByTeamBuilder<(T0, T1)> {
            /// Finish writing the builder to get an [Offset](::planus::Offset) to a serialized [DropshotTilesByTeam].
            #[inline]
            pub fn finish(self, builder: &mut ::planus::Builder) -> ::planus::Offset<DropshotTilesByTeam>
            where
                Self: ::planus::WriteAsOffset<DropshotTilesByTeam>,
            {
                ::planus::WriteAsOffset::prepare(&self, builder)
            }
        }

        impl<
            T0: ::planus::WriteAs<::planus::Offset<[::planus::Offset<self::DropshotTile>]>>,
            T1: ::planus::WriteAs<::planus::Offset<[::planus::Offset<self::DropshotTile>]>>,
        > ::planus::WriteAs<::planus::Offset<DropshotTilesByTeam>> for DropshotTilesByTeamBuilder<(T0, T1)>
        {
            type Prepared = ::planus::Offset<DropshotTilesByTeam>;

            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<DropshotTilesByTeam> {
                ::planus::WriteAsOffset::prepare(self, builder)
            }
        }

        impl<
            T0: ::planus::WriteAs<::planus::Offset<[::planus::Offset<self::DropshotTile>]>>,
            T1: ::planus::WriteAs<::planus::Offset<[::planus::Offset<self::DropshotTile>]>>,
        > ::planus::WriteAsOptional<::planus::Offset<DropshotTilesByTeam>> for DropshotTilesByTeamBuilder<(T0, T1)>
        {
            type Prepared = ::planus::Offset<DropshotTilesByTeam>;

            #[inline]
            fn prepare(
                &self,
                builder: &mut ::planus::Builder,
            ) -> ::core::option::Option<::planus::Offset<DropshotTilesByTeam>> {
                ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
            }
        }

        impl<
            T0: ::planus::WriteAs<::planus::Offset<[::planus::Offset<self::DropshotTile>]>>,
            T1: ::planus::WriteAs<::planus::Offset<[::planus::Offset<self::DropshotTile>]>>,
        > ::planus::WriteAsOffset<DropshotTilesByTeam> for DropshotTilesByTeamBuilder<(T0, T1)>
        {
            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<DropshotTilesByTeam> {
                let (v0, v1) = &self.0;
                DropshotTilesByTeam::create(builder, v0, v1)
            }
        }

        /// Reference to a deserialized [DropshotTilesByTeam].
        #[derive(Copy, Clone)]
        pub struct DropshotTilesByTeamRef<'a>(#[allow(dead_code)] ::planus::table_reader::Table<'a>);

        impl<'a> DropshotTilesByTeamRef<'a> {
            /// Getter for the [`blue_tiles` field](DropshotTilesByTeam#structfield.blue_tiles).
            #[inline]
            pub fn blue_tiles(&self) -> ::planus::Result<::planus::Vector<'a, ::planus::Result<self::DropshotTileRef<'a>>>> {
                self.0.access_required(0, "DropshotTilesByTeam", "blue_tiles")
            }

            /// Getter for the [`orange_tiles` field](DropshotTilesByTeam#structfield.orange_tiles).
            #[inline]
            pub fn orange_tiles(
                &self,
            ) -> ::planus::Result<::planus::Vector<'a, ::planus::Result<self::DropshotTileRef<'a>>>> {
                self.0.access_required(1, "DropshotTilesByTeam", "orange_tiles")
            }
        }

        impl<'a> ::core::fmt::Debug for DropshotTilesByTeamRef<'a> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                let mut f = f.debug_struct("DropshotTilesByTeamRef");
                f.field("blue_tiles", &self.blue_tiles());
                f.field("orange_tiles", &self.orange_tiles());
                f.finish()
            }
        }

        impl<'a> ::core::convert::TryFrom<DropshotTilesByTeamRef<'a>> for DropshotTilesByTeam {
            type Error = ::planus::Error;

            #[allow(unreachable_code)]
            fn try_from(value: DropshotTilesByTeamRef<'a>) -> ::planus::Result<Self> {
                ::core::result::Result::Ok(Self {
                    blue_tiles: value.blue_tiles()?.to_vec_result()?,
                    orange_tiles: value.orange_tiles()?.to_vec_result()?,
                })
            }
        }

        impl<'a> ::planus::TableRead<'a> for DropshotTilesByTeamRef<'a> {
            #[inline]
            fn from_buffer(
                buffer: ::planus::SliceWithStartOffset<'a>,
                offset: usize,
            ) -> ::core::result::Result<Self, ::planus::errors::ErrorKind> {
                ::core::result::Result::Ok(Self(::planus::table_reader::Table::from_buffer(buffer, offset)?))
            }
        }

        impl<'a> ::planus::VectorReadInner<'a> for DropshotTilesByTeamRef<'a> {
            type Error = ::planus::Error;
            const STRIDE: usize = 4;

            unsafe fn from_buffer(buffer: ::planus::SliceWithStartOffset<'a>, offset: usize) -> ::planus::Result<Self> {
                ::planus::TableRead::from_buffer(buffer, offset).map_err(|error_kind| {
                    error_kind.with_error_location("[DropshotTilesByTeamRef]", "get", buffer.offset_from_start)
                })
            }
        }

        /// # Safety
        /// The planus compiler generates implementations that initialize
        /// the bytes in `write_values`.
        unsafe impl ::planus::VectorWrite<::planus::Offset<DropshotTilesByTeam>> for DropshotTilesByTeam {
            type Value = ::planus::Offset<DropshotTilesByTeam>;
            const STRIDE: usize = 4;
            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> Self::Value {
                ::planus::WriteAs::prepare(self, builder)
            }

            #[inline]
            unsafe fn write_values(
                values: &[::planus::Offset<DropshotTilesByTeam>],
                bytes: *mut ::core::mem::MaybeUninit<u8>,
                buffer_position: u32,
            ) {
                let bytes = bytes as *mut [::core::mem::MaybeUninit<u8>; 4];
                for (i, v) in ::core::iter::Iterator::enumerate(values.iter()) {
                    ::planus::WriteAsPrimitive::write(
                        v,
                        ::planus::Cursor::new(unsafe { &mut *bytes.add(i) }),
                        buffer_position - (Self::STRIDE * i) as u32,
                    );
                }
            }
        }

        impl<'a> ::planus::ReadAsRoot<'a> for DropshotTilesByTeamRef<'a> {
            fn read_as_root(slice: &'a [u8]) -> ::planus::Result<Self> {
                ::planus::TableRead::from_buffer(
                    ::planus::SliceWithStartOffset {
                        buffer: slice,
                        offset_from_start: 0,
                    },
                    0,
                )
                .map_err(|error_kind| error_kind.with_error_location("[DropshotTilesByTeamRef]", "read_as_root", 0))
            }
        }

        /// The struct `WheelsWithContact` in the namespace `rocketsim`
        ///
        /// Generated from these locations:
        /// * Struct `WheelsWithContact` in the file `spec/game_state.fbs:126`
        #[derive(
            Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Default, ::serde::Serialize, ::serde::Deserialize,
        )]
        pub struct WheelsWithContact {
            /// The field `front_left` in the struct `WheelsWithContact`
            pub front_left: bool,

            /// The field `front_right` in the struct `WheelsWithContact`
            pub front_right: bool,

            /// The field `rear_left` in the struct `WheelsWithContact`
            pub rear_left: bool,

            /// The field `rear_right` in the struct `WheelsWithContact`
            pub rear_right: bool,
        }

        /// # Safety
        /// The Planus compiler correctly calculates `ALIGNMENT` and `SIZE`.
        unsafe impl ::planus::Primitive for WheelsWithContact {
            const ALIGNMENT: usize = 1;
            const SIZE: usize = 4;
        }

        #[allow(clippy::identity_op)]
        impl ::planus::WriteAsPrimitive<WheelsWithContact> for WheelsWithContact {
            #[inline]
            fn write<const N: usize>(&self, cursor: ::planus::Cursor<'_, N>, buffer_position: u32) {
                let (cur, cursor) = cursor.split::<1, 3>();
                self.front_left.write(cur, buffer_position - 0);
                let (cur, cursor) = cursor.split::<1, 2>();
                self.front_right.write(cur, buffer_position - 1);
                let (cur, cursor) = cursor.split::<1, 1>();
                self.rear_left.write(cur, buffer_position - 2);
                let (cur, cursor) = cursor.split::<1, 0>();
                self.rear_right.write(cur, buffer_position - 3);
                cursor.finish([]);
            }
        }

        impl ::planus::WriteAsOffset<WheelsWithContact> for WheelsWithContact {
            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<WheelsWithContact> {
                unsafe {
                    builder.write_with(4, 0, |buffer_position, bytes| {
                        let bytes = bytes.as_mut_ptr();

                        ::planus::WriteAsPrimitive::write(
                            self,
                            ::planus::Cursor::new(&mut *(bytes as *mut [::core::mem::MaybeUninit<u8>; 4])),
                            buffer_position,
                        );
                    });
                }
                builder.current_offset()
            }
        }

        impl ::planus::WriteAs<WheelsWithContact> for WheelsWithContact {
            type Prepared = Self;
            #[inline]
            fn prepare(&self, _builder: &mut ::planus::Builder) -> Self {
                *self
            }
        }

        impl ::planus::WriteAsOptional<WheelsWithContact> for WheelsWithContact {
            type Prepared = Self;
            #[inline]
            fn prepare(&self, _builder: &mut ::planus::Builder) -> ::core::option::Option<Self> {
                ::core::option::Option::Some(*self)
            }
        }

        /// Reference to a deserialized [WheelsWithContact].
        #[derive(Copy, Clone)]
        pub struct WheelsWithContactRef<'a>(::planus::ArrayWithStartOffset<'a, 4>);

        impl<'a> WheelsWithContactRef<'a> {
            /// Getter for the [`front_left` field](WheelsWithContact#structfield.front_left).
            pub fn front_left(&self) -> bool {
                let buffer = self.0.advance_as_array::<1>(0).unwrap();

                buffer.as_array()[0] != 0
            }

            /// Getter for the [`front_right` field](WheelsWithContact#structfield.front_right).
            pub fn front_right(&self) -> bool {
                let buffer = self.0.advance_as_array::<1>(1).unwrap();

                buffer.as_array()[0] != 0
            }

            /// Getter for the [`rear_left` field](WheelsWithContact#structfield.rear_left).
            pub fn rear_left(&self) -> bool {
                let buffer = self.0.advance_as_array::<1>(2).unwrap();

                buffer.as_array()[0] != 0
            }

            /// Getter for the [`rear_right` field](WheelsWithContact#structfield.rear_right).
            pub fn rear_right(&self) -> bool {
                let buffer = self.0.advance_as_array::<1>(3).unwrap();

                buffer.as_array()[0] != 0
            }
        }

        impl<'a> ::core::fmt::Debug for WheelsWithContactRef<'a> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                let mut f = f.debug_struct("WheelsWithContactRef");
                f.field("front_left", &self.front_left());
                f.field("front_right", &self.front_right());
                f.field("rear_left", &self.rear_left());
                f.field("rear_right", &self.rear_right());
                f.finish()
            }
        }

        impl<'a> ::core::convert::From<::planus::ArrayWithStartOffset<'a, 4>> for WheelsWithContactRef<'a> {
            fn from(array: ::planus::ArrayWithStartOffset<'a, 4>) -> Self {
                Self(array)
            }
        }

        impl<'a> ::core::convert::From<WheelsWithContactRef<'a>> for WheelsWithContact {
            #[allow(unreachable_code)]
            fn from(value: WheelsWithContactRef<'a>) -> Self {
                Self {
                    front_left: value.front_left(),
                    front_right: value.front_right(),
                    rear_left: value.rear_left(),
                    rear_right: value.rear_right(),
                }
            }
        }

        impl<'a, 'b> ::core::cmp::PartialEq<WheelsWithContactRef<'a>> for WheelsWithContactRef<'b> {
            fn eq(&self, other: &WheelsWithContactRef<'_>) -> bool {
                self.front_left() == other.front_left()
                    && self.front_right() == other.front_right()
                    && self.rear_left() == other.rear_left()
                    && self.rear_right() == other.rear_right()
            }
        }

        impl<'a> ::core::cmp::Eq for WheelsWithContactRef<'a> {}
        impl<'a, 'b> ::core::cmp::PartialOrd<WheelsWithContactRef<'a>> for WheelsWithContactRef<'b> {
            fn partial_cmp(&self, other: &WheelsWithContactRef<'_>) -> ::core::option::Option<::core::cmp::Ordering> {
                ::core::option::Option::Some(::core::cmp::Ord::cmp(self, other))
            }
        }

        impl<'a> ::core::cmp::Ord for WheelsWithContactRef<'a> {
            fn cmp(&self, other: &WheelsWithContactRef<'_>) -> ::core::cmp::Ordering {
                self.front_left()
                    .cmp(&other.front_left())
                    .then_with(|| self.front_right().cmp(&other.front_right()))
                    .then_with(|| self.rear_left().cmp(&other.rear_left()))
                    .then_with(|| self.rear_right().cmp(&other.rear_right()))
            }
        }

        impl<'a> ::core::hash::Hash for WheelsWithContactRef<'a> {
            fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
                self.front_left().hash(state);
                self.front_right().hash(state);
                self.rear_left().hash(state);
                self.rear_right().hash(state);
            }
        }

        impl<'a> ::planus::TableRead<'a> for WheelsWithContactRef<'a> {
            #[inline]
            fn from_buffer(
                buffer: ::planus::SliceWithStartOffset<'a>,
                offset: usize,
            ) -> ::core::result::Result<Self, ::planus::errors::ErrorKind> {
                let buffer = buffer.advance_as_array::<4>(offset)?;
                ::core::result::Result::Ok(Self(buffer))
            }
        }

        impl<'a> ::planus::VectorRead<'a> for WheelsWithContactRef<'a> {
            const STRIDE: usize = 4;

            #[inline]
            unsafe fn from_buffer(buffer: ::planus::SliceWithStartOffset<'a>, offset: usize) -> Self {
                Self(unsafe { buffer.unchecked_advance_as_array(offset) })
            }
        }

        /// # Safety
        /// The planus compiler generates implementations that initialize
        /// the bytes in `write_values`.
        unsafe impl ::planus::VectorWrite<WheelsWithContact> for WheelsWithContact {
            const STRIDE: usize = 4;

            type Value = WheelsWithContact;

            #[inline]
            fn prepare(&self, _builder: &mut ::planus::Builder) -> Self::Value {
                *self
            }

            #[inline]
            unsafe fn write_values(
                values: &[WheelsWithContact],
                bytes: *mut ::core::mem::MaybeUninit<u8>,
                buffer_position: u32,
            ) {
                let bytes = bytes as *mut [::core::mem::MaybeUninit<u8>; 4];
                for (i, v) in ::core::iter::Iterator::enumerate(values.iter()) {
                    ::planus::WriteAsPrimitive::write(
                        v,
                        ::planus::Cursor::new(unsafe { &mut *bytes.add(i) }),
                        buffer_position - (4 * i) as u32,
                    );
                }
            }
        }

        /// The table `CarState` in the namespace `rocketsim`
        ///
        /// Generated from these locations:
        /// * Table `CarState` in the file `spec/game_state.fbs:133`
        #[derive(Clone, Debug, PartialEq, PartialOrd, ::serde::Serialize, ::serde::Deserialize)]
        pub struct CarState {
            /// The field `physics` in the table `CarState`
            pub physics: self::PhysState,
            /// The field `is_on_ground` in the table `CarState`
            pub is_on_ground: bool,
            /// The field `wheels_with_contact` in the table `CarState`
            pub wheels_with_contact: self::WheelsWithContact,
            /// The field `has_jumped` in the table `CarState`
            pub has_jumped: bool,
            /// The field `has_double_jumped` in the table `CarState`
            pub has_double_jumped: bool,
            /// The field `has_flipped` in the table `CarState`
            pub has_flipped: bool,
            /// The field `flip_rel_torque` in the table `CarState`
            pub flip_rel_torque: self::Vec3,
            /// The field `jump_time` in the table `CarState`
            pub jump_time: f32,
            /// The field `flip_time` in the table `CarState`
            pub flip_time: f32,
            /// The field `is_flipping` in the table `CarState`
            pub is_flipping: bool,
            /// The field `is_jumping` in the table `CarState`
            pub is_jumping: bool,
            /// The field `air_time` in the table `CarState`
            pub air_time: f32,
            /// The field `air_time_since_jump` in the table `CarState`
            pub air_time_since_jump: f32,
            /// The field `boost` in the table `CarState`
            pub boost: f32,
            /// The field `time_since_boosted` in the table `CarState`
            pub time_since_boosted: f32,
            /// The field `is_boosting` in the table `CarState`
            pub is_boosting: bool,
            /// The field `boosting_time` in the table `CarState`
            pub boosting_time: f32,
            /// The field `is_supersonic` in the table `CarState`
            pub is_supersonic: bool,
            /// The field `supersonic_time` in the table `CarState`
            pub supersonic_time: f32,
            /// The field `handbrake_val` in the table `CarState`
            pub handbrake_val: f32,
            /// The field `is_auto_flipping` in the table `CarState`
            pub is_auto_flipping: bool,
            /// The field `auto_flip_timer` in the table `CarState`
            pub auto_flip_timer: f32,
            /// The field `auto_flip_torque_scale` in the table `CarState`
            pub auto_flip_torque_scale: f32,
            /// The field `world_contact_normal` in the table `CarState`
            pub world_contact_normal: ::core::option::Option<self::Vec3>,
            /// The field `car_contact` in the table `CarState`
            pub car_contact: ::core::option::Option<::planus::alloc::boxed::Box<self::CarContact>>,
            /// The field `is_demoed` in the table `CarState`
            pub is_demoed: bool,
            /// The field `demo_respawn_timer` in the table `CarState`
            pub demo_respawn_timer: f32,
            /// The field `ball_hit_info` in the table `CarState`
            pub ball_hit_info: ::core::option::Option<::planus::alloc::boxed::Box<self::BallHitInfo>>,
            /// The field `last_controls` in the table `CarState`
            pub last_controls: self::CarControls,
        }

        #[allow(clippy::derivable_impls)]
        impl ::core::default::Default for CarState {
            fn default() -> Self {
                Self {
                    physics: ::core::default::Default::default(),
                    is_on_ground: false,
                    wheels_with_contact: ::core::default::Default::default(),
                    has_jumped: false,
                    has_double_jumped: false,
                    has_flipped: false,
                    flip_rel_torque: ::core::default::Default::default(),
                    jump_time: 0.0,
                    flip_time: 0.0,
                    is_flipping: false,
                    is_jumping: false,
                    air_time: 0.0,
                    air_time_since_jump: 0.0,
                    boost: 0.0,
                    time_since_boosted: 0.0,
                    is_boosting: false,
                    boosting_time: 0.0,
                    is_supersonic: false,
                    supersonic_time: 0.0,
                    handbrake_val: 0.0,
                    is_auto_flipping: false,
                    auto_flip_timer: 0.0,
                    auto_flip_torque_scale: 0.0,
                    world_contact_normal: ::core::default::Default::default(),
                    car_contact: ::core::default::Default::default(),
                    is_demoed: false,
                    demo_respawn_timer: 0.0,
                    ball_hit_info: ::core::default::Default::default(),
                    last_controls: ::core::default::Default::default(),
                }
            }
        }

        impl CarState {
            /// Creates a [CarStateBuilder] for serializing an instance of this table.
            #[inline]
            pub fn builder() -> CarStateBuilder<()> {
                CarStateBuilder(())
            }

            #[allow(clippy::too_many_arguments)]
            pub fn create(
                builder: &mut ::planus::Builder,
                field_physics: impl ::planus::WriteAs<self::PhysState>,
                field_is_on_ground: impl ::planus::WriteAsDefault<bool, bool>,
                field_wheels_with_contact: impl ::planus::WriteAs<self::WheelsWithContact>,
                field_has_jumped: impl ::planus::WriteAsDefault<bool, bool>,
                field_has_double_jumped: impl ::planus::WriteAsDefault<bool, bool>,
                field_has_flipped: impl ::planus::WriteAsDefault<bool, bool>,
                field_flip_rel_torque: impl ::planus::WriteAs<self::Vec3>,
                field_jump_time: impl ::planus::WriteAsDefault<f32, f32>,
                field_flip_time: impl ::planus::WriteAsDefault<f32, f32>,
                field_is_flipping: impl ::planus::WriteAsDefault<bool, bool>,
                field_is_jumping: impl ::planus::WriteAsDefault<bool, bool>,
                field_air_time: impl ::planus::WriteAsDefault<f32, f32>,
                field_air_time_since_jump: impl ::planus::WriteAsDefault<f32, f32>,
                field_boost: impl ::planus::WriteAsDefault<f32, f32>,
                field_time_since_boosted: impl ::planus::WriteAsDefault<f32, f32>,
                field_is_boosting: impl ::planus::WriteAsDefault<bool, bool>,
                field_boosting_time: impl ::planus::WriteAsDefault<f32, f32>,
                field_is_supersonic: impl ::planus::WriteAsDefault<bool, bool>,
                field_supersonic_time: impl ::planus::WriteAsDefault<f32, f32>,
                field_handbrake_val: impl ::planus::WriteAsDefault<f32, f32>,
                field_is_auto_flipping: impl ::planus::WriteAsDefault<bool, bool>,
                field_auto_flip_timer: impl ::planus::WriteAsDefault<f32, f32>,
                field_auto_flip_torque_scale: impl ::planus::WriteAsDefault<f32, f32>,
                field_world_contact_normal: impl ::planus::WriteAsOptional<self::Vec3>,
                field_car_contact: impl ::planus::WriteAsOptional<::planus::Offset<self::CarContact>>,
                field_is_demoed: impl ::planus::WriteAsDefault<bool, bool>,
                field_demo_respawn_timer: impl ::planus::WriteAsDefault<f32, f32>,
                field_ball_hit_info: impl ::planus::WriteAsOptional<::planus::Offset<self::BallHitInfo>>,
                field_last_controls: impl ::planus::WriteAs<self::CarControls>,
            ) -> ::planus::Offset<Self> {
                let prepared_physics = field_physics.prepare(builder);
                let prepared_is_on_ground = field_is_on_ground.prepare(builder, &false);
                let prepared_wheels_with_contact = field_wheels_with_contact.prepare(builder);
                let prepared_has_jumped = field_has_jumped.prepare(builder, &false);
                let prepared_has_double_jumped = field_has_double_jumped.prepare(builder, &false);
                let prepared_has_flipped = field_has_flipped.prepare(builder, &false);
                let prepared_flip_rel_torque = field_flip_rel_torque.prepare(builder);
                let prepared_jump_time = field_jump_time.prepare(builder, &0.0);
                let prepared_flip_time = field_flip_time.prepare(builder, &0.0);
                let prepared_is_flipping = field_is_flipping.prepare(builder, &false);
                let prepared_is_jumping = field_is_jumping.prepare(builder, &false);
                let prepared_air_time = field_air_time.prepare(builder, &0.0);
                let prepared_air_time_since_jump = field_air_time_since_jump.prepare(builder, &0.0);
                let prepared_boost = field_boost.prepare(builder, &0.0);
                let prepared_time_since_boosted = field_time_since_boosted.prepare(builder, &0.0);
                let prepared_is_boosting = field_is_boosting.prepare(builder, &false);
                let prepared_boosting_time = field_boosting_time.prepare(builder, &0.0);
                let prepared_is_supersonic = field_is_supersonic.prepare(builder, &false);
                let prepared_supersonic_time = field_supersonic_time.prepare(builder, &0.0);
                let prepared_handbrake_val = field_handbrake_val.prepare(builder, &0.0);
                let prepared_is_auto_flipping = field_is_auto_flipping.prepare(builder, &false);
                let prepared_auto_flip_timer = field_auto_flip_timer.prepare(builder, &0.0);
                let prepared_auto_flip_torque_scale = field_auto_flip_torque_scale.prepare(builder, &0.0);
                let prepared_world_contact_normal = field_world_contact_normal.prepare(builder);
                let prepared_car_contact = field_car_contact.prepare(builder);
                let prepared_is_demoed = field_is_demoed.prepare(builder, &false);
                let prepared_demo_respawn_timer = field_demo_respawn_timer.prepare(builder, &0.0);
                let prepared_ball_hit_info = field_ball_hit_info.prepare(builder);
                let prepared_last_controls = field_last_controls.prepare(builder);

                let mut table_writer: ::planus::table_writer::TableWriter<62> = ::core::default::Default::default();
                table_writer.write_entry::<self::PhysState>(0);
                table_writer.write_entry::<self::Vec3>(6);
                if prepared_jump_time.is_some() {
                    table_writer.write_entry::<f32>(7);
                }
                if prepared_flip_time.is_some() {
                    table_writer.write_entry::<f32>(8);
                }
                if prepared_air_time.is_some() {
                    table_writer.write_entry::<f32>(11);
                }
                if prepared_air_time_since_jump.is_some() {
                    table_writer.write_entry::<f32>(12);
                }
                if prepared_boost.is_some() {
                    table_writer.write_entry::<f32>(13);
                }
                if prepared_time_since_boosted.is_some() {
                    table_writer.write_entry::<f32>(14);
                }
                if prepared_boosting_time.is_some() {
                    table_writer.write_entry::<f32>(16);
                }
                if prepared_supersonic_time.is_some() {
                    table_writer.write_entry::<f32>(18);
                }
                if prepared_handbrake_val.is_some() {
                    table_writer.write_entry::<f32>(19);
                }
                if prepared_auto_flip_timer.is_some() {
                    table_writer.write_entry::<f32>(21);
                }
                if prepared_auto_flip_torque_scale.is_some() {
                    table_writer.write_entry::<f32>(22);
                }
                if prepared_world_contact_normal.is_some() {
                    table_writer.write_entry::<self::Vec3>(23);
                }
                if prepared_car_contact.is_some() {
                    table_writer.write_entry::<::planus::Offset<self::CarContact>>(24);
                }
                if prepared_demo_respawn_timer.is_some() {
                    table_writer.write_entry::<f32>(26);
                }
                if prepared_ball_hit_info.is_some() {
                    table_writer.write_entry::<::planus::Offset<self::BallHitInfo>>(27);
                }
                table_writer.write_entry::<self::CarControls>(28);
                if prepared_is_on_ground.is_some() {
                    table_writer.write_entry::<bool>(1);
                }
                table_writer.write_entry::<self::WheelsWithContact>(2);
                if prepared_has_jumped.is_some() {
                    table_writer.write_entry::<bool>(3);
                }
                if prepared_has_double_jumped.is_some() {
                    table_writer.write_entry::<bool>(4);
                }
                if prepared_has_flipped.is_some() {
                    table_writer.write_entry::<bool>(5);
                }
                if prepared_is_flipping.is_some() {
                    table_writer.write_entry::<bool>(9);
                }
                if prepared_is_jumping.is_some() {
                    table_writer.write_entry::<bool>(10);
                }
                if prepared_is_boosting.is_some() {
                    table_writer.write_entry::<bool>(15);
                }
                if prepared_is_supersonic.is_some() {
                    table_writer.write_entry::<bool>(17);
                }
                if prepared_is_auto_flipping.is_some() {
                    table_writer.write_entry::<bool>(20);
                }
                if prepared_is_demoed.is_some() {
                    table_writer.write_entry::<bool>(25);
                }

                unsafe {
                    table_writer.finish(builder, |object_writer| {
                        object_writer.write::<_, _, 72>(&prepared_physics);
                        object_writer.write::<_, _, 12>(&prepared_flip_rel_torque);
                        if let ::core::option::Option::Some(prepared_jump_time) = prepared_jump_time {
                            object_writer.write::<_, _, 4>(&prepared_jump_time);
                        }
                        if let ::core::option::Option::Some(prepared_flip_time) = prepared_flip_time {
                            object_writer.write::<_, _, 4>(&prepared_flip_time);
                        }
                        if let ::core::option::Option::Some(prepared_air_time) = prepared_air_time {
                            object_writer.write::<_, _, 4>(&prepared_air_time);
                        }
                        if let ::core::option::Option::Some(prepared_air_time_since_jump) = prepared_air_time_since_jump {
                            object_writer.write::<_, _, 4>(&prepared_air_time_since_jump);
                        }
                        if let ::core::option::Option::Some(prepared_boost) = prepared_boost {
                            object_writer.write::<_, _, 4>(&prepared_boost);
                        }
                        if let ::core::option::Option::Some(prepared_time_since_boosted) = prepared_time_since_boosted {
                            object_writer.write::<_, _, 4>(&prepared_time_since_boosted);
                        }
                        if let ::core::option::Option::Some(prepared_boosting_time) = prepared_boosting_time {
                            object_writer.write::<_, _, 4>(&prepared_boosting_time);
                        }
                        if let ::core::option::Option::Some(prepared_supersonic_time) = prepared_supersonic_time {
                            object_writer.write::<_, _, 4>(&prepared_supersonic_time);
                        }
                        if let ::core::option::Option::Some(prepared_handbrake_val) = prepared_handbrake_val {
                            object_writer.write::<_, _, 4>(&prepared_handbrake_val);
                        }
                        if let ::core::option::Option::Some(prepared_auto_flip_timer) = prepared_auto_flip_timer {
                            object_writer.write::<_, _, 4>(&prepared_auto_flip_timer);
                        }
                        if let ::core::option::Option::Some(prepared_auto_flip_torque_scale) =
                            prepared_auto_flip_torque_scale
                        {
                            object_writer.write::<_, _, 4>(&prepared_auto_flip_torque_scale);
                        }
                        if let ::core::option::Option::Some(prepared_world_contact_normal) = prepared_world_contact_normal {
                            object_writer.write::<_, _, 12>(&prepared_world_contact_normal);
                        }
                        if let ::core::option::Option::Some(prepared_car_contact) = prepared_car_contact {
                            object_writer.write::<_, _, 4>(&prepared_car_contact);
                        }
                        if let ::core::option::Option::Some(prepared_demo_respawn_timer) = prepared_demo_respawn_timer {
                            object_writer.write::<_, _, 4>(&prepared_demo_respawn_timer);
                        }
                        if let ::core::option::Option::Some(prepared_ball_hit_info) = prepared_ball_hit_info {
                            object_writer.write::<_, _, 4>(&prepared_ball_hit_info);
                        }
                        object_writer.write::<_, _, 24>(&prepared_last_controls);
                        if let ::core::option::Option::Some(prepared_is_on_ground) = prepared_is_on_ground {
                            object_writer.write::<_, _, 1>(&prepared_is_on_ground);
                        }
                        object_writer.write::<_, _, 4>(&prepared_wheels_with_contact);
                        if let ::core::option::Option::Some(prepared_has_jumped) = prepared_has_jumped {
                            object_writer.write::<_, _, 1>(&prepared_has_jumped);
                        }
                        if let ::core::option::Option::Some(prepared_has_double_jumped) = prepared_has_double_jumped {
                            object_writer.write::<_, _, 1>(&prepared_has_double_jumped);
                        }
                        if let ::core::option::Option::Some(prepared_has_flipped) = prepared_has_flipped {
                            object_writer.write::<_, _, 1>(&prepared_has_flipped);
                        }
                        if let ::core::option::Option::Some(prepared_is_flipping) = prepared_is_flipping {
                            object_writer.write::<_, _, 1>(&prepared_is_flipping);
                        }
                        if let ::core::option::Option::Some(prepared_is_jumping) = prepared_is_jumping {
                            object_writer.write::<_, _, 1>(&prepared_is_jumping);
                        }
                        if let ::core::option::Option::Some(prepared_is_boosting) = prepared_is_boosting {
                            object_writer.write::<_, _, 1>(&prepared_is_boosting);
                        }
                        if let ::core::option::Option::Some(prepared_is_supersonic) = prepared_is_supersonic {
                            object_writer.write::<_, _, 1>(&prepared_is_supersonic);
                        }
                        if let ::core::option::Option::Some(prepared_is_auto_flipping) = prepared_is_auto_flipping {
                            object_writer.write::<_, _, 1>(&prepared_is_auto_flipping);
                        }
                        if let ::core::option::Option::Some(prepared_is_demoed) = prepared_is_demoed {
                            object_writer.write::<_, _, 1>(&prepared_is_demoed);
                        }
                    });
                }
                builder.current_offset()
            }
        }

        impl ::planus::WriteAs<::planus::Offset<CarState>> for CarState {
            type Prepared = ::planus::Offset<Self>;

            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<CarState> {
                ::planus::WriteAsOffset::prepare(self, builder)
            }
        }

        impl ::planus::WriteAsOptional<::planus::Offset<CarState>> for CarState {
            type Prepared = ::planus::Offset<Self>;

            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::core::option::Option<::planus::Offset<CarState>> {
                ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
            }
        }

        impl ::planus::WriteAsOffset<CarState> for CarState {
            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<CarState> {
                CarState::create(
                    builder,
                    self.physics,
                    self.is_on_ground,
                    self.wheels_with_contact,
                    self.has_jumped,
                    self.has_double_jumped,
                    self.has_flipped,
                    self.flip_rel_torque,
                    self.jump_time,
                    self.flip_time,
                    self.is_flipping,
                    self.is_jumping,
                    self.air_time,
                    self.air_time_since_jump,
                    self.boost,
                    self.time_since_boosted,
                    self.is_boosting,
                    self.boosting_time,
                    self.is_supersonic,
                    self.supersonic_time,
                    self.handbrake_val,
                    self.is_auto_flipping,
                    self.auto_flip_timer,
                    self.auto_flip_torque_scale,
                    self.world_contact_normal,
                    &self.car_contact,
                    self.is_demoed,
                    self.demo_respawn_timer,
                    &self.ball_hit_info,
                    self.last_controls,
                )
            }
        }

        /// Builder for serializing an instance of the [CarState] type.
        ///
        /// Can be created using the [CarState::builder] method.
        #[derive(Debug)]
        #[must_use]
        pub struct CarStateBuilder<State>(State);

        impl CarStateBuilder<()> {
            /// Setter for the [`physics` field](CarState#structfield.physics).
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn physics<T0>(self, value: T0) -> CarStateBuilder<(T0,)>
            where
                T0: ::planus::WriteAs<self::PhysState>,
            {
                CarStateBuilder((value,))
            }
        }

        impl<T0> CarStateBuilder<(T0,)> {
            /// Setter for the [`is_on_ground` field](CarState#structfield.is_on_ground).
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn is_on_ground<T1>(self, value: T1) -> CarStateBuilder<(T0, T1)>
            where
                T1: ::planus::WriteAsDefault<bool, bool>,
            {
                let (v0,) = self.0;
                CarStateBuilder((v0, value))
            }

            /// Sets the [`is_on_ground` field](CarState#structfield.is_on_ground) to the default value.
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn is_on_ground_as_default(self) -> CarStateBuilder<(T0, ::planus::DefaultValue)> {
                self.is_on_ground(::planus::DefaultValue)
            }
        }

        impl<T0, T1> CarStateBuilder<(T0, T1)> {
            /// Setter for the [`wheels_with_contact` field](CarState#structfield.wheels_with_contact).
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn wheels_with_contact<T2>(self, value: T2) -> CarStateBuilder<(T0, T1, T2)>
            where
                T2: ::planus::WriteAs<self::WheelsWithContact>,
            {
                let (v0, v1) = self.0;
                CarStateBuilder((v0, v1, value))
            }
        }

        impl<T0, T1, T2> CarStateBuilder<(T0, T1, T2)> {
            /// Setter for the [`has_jumped` field](CarState#structfield.has_jumped).
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn has_jumped<T3>(self, value: T3) -> CarStateBuilder<(T0, T1, T2, T3)>
            where
                T3: ::planus::WriteAsDefault<bool, bool>,
            {
                let (v0, v1, v2) = self.0;
                CarStateBuilder((v0, v1, v2, value))
            }

            /// Sets the [`has_jumped` field](CarState#structfield.has_jumped) to the default value.
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn has_jumped_as_default(self) -> CarStateBuilder<(T0, T1, T2, ::planus::DefaultValue)> {
                self.has_jumped(::planus::DefaultValue)
            }
        }

        impl<T0, T1, T2, T3> CarStateBuilder<(T0, T1, T2, T3)> {
            /// Setter for the [`has_double_jumped` field](CarState#structfield.has_double_jumped).
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn has_double_jumped<T4>(self, value: T4) -> CarStateBuilder<(T0, T1, T2, T3, T4)>
            where
                T4: ::planus::WriteAsDefault<bool, bool>,
            {
                let (v0, v1, v2, v3) = self.0;
                CarStateBuilder((v0, v1, v2, v3, value))
            }

            /// Sets the [`has_double_jumped` field](CarState#structfield.has_double_jumped) to the default value.
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn has_double_jumped_as_default(self) -> CarStateBuilder<(T0, T1, T2, T3, ::planus::DefaultValue)> {
                self.has_double_jumped(::planus::DefaultValue)
            }
        }

        impl<T0, T1, T2, T3, T4> CarStateBuilder<(T0, T1, T2, T3, T4)> {
            /// Setter for the [`has_flipped` field](CarState#structfield.has_flipped).
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn has_flipped<T5>(self, value: T5) -> CarStateBuilder<(T0, T1, T2, T3, T4, T5)>
            where
                T5: ::planus::WriteAsDefault<bool, bool>,
            {
                let (v0, v1, v2, v3, v4) = self.0;
                CarStateBuilder((v0, v1, v2, v3, v4, value))
            }

            /// Sets the [`has_flipped` field](CarState#structfield.has_flipped) to the default value.
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn has_flipped_as_default(self) -> CarStateBuilder<(T0, T1, T2, T3, T4, ::planus::DefaultValue)> {
                self.has_flipped(::planus::DefaultValue)
            }
        }

        impl<T0, T1, T2, T3, T4, T5> CarStateBuilder<(T0, T1, T2, T3, T4, T5)> {
            /// Setter for the [`flip_rel_torque` field](CarState#structfield.flip_rel_torque).
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn flip_rel_torque<T6>(self, value: T6) -> CarStateBuilder<(T0, T1, T2, T3, T4, T5, T6)>
            where
                T6: ::planus::WriteAs<self::Vec3>,
            {
                let (v0, v1, v2, v3, v4, v5) = self.0;
                CarStateBuilder((v0, v1, v2, v3, v4, v5, value))
            }
        }

        impl<T0, T1, T2, T3, T4, T5, T6> CarStateBuilder<(T0, T1, T2, T3, T4, T5, T6)> {
            /// Setter for the [`jump_time` field](CarState#structfield.jump_time).
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn jump_time<T7>(self, value: T7) -> CarStateBuilder<(T0, T1, T2, T3, T4, T5, T6, T7)>
            where
                T7: ::planus::WriteAsDefault<f32, f32>,
            {
                let (v0, v1, v2, v3, v4, v5, v6) = self.0;
                CarStateBuilder((v0, v1, v2, v3, v4, v5, v6, value))
            }

            /// Sets the [`jump_time` field](CarState#structfield.jump_time) to the default value.
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn jump_time_as_default(self) -> CarStateBuilder<(T0, T1, T2, T3, T4, T5, T6, ::planus::DefaultValue)> {
                self.jump_time(::planus::DefaultValue)
            }
        }

        impl<T0, T1, T2, T3, T4, T5, T6, T7> CarStateBuilder<(T0, T1, T2, T3, T4, T5, T6, T7)> {
            /// Setter for the [`flip_time` field](CarState#structfield.flip_time).
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn flip_time<T8>(self, value: T8) -> CarStateBuilder<(T0, T1, T2, T3, T4, T5, T6, T7, T8)>
            where
                T8: ::planus::WriteAsDefault<f32, f32>,
            {
                let (v0, v1, v2, v3, v4, v5, v6, v7) = self.0;
                CarStateBuilder((v0, v1, v2, v3, v4, v5, v6, v7, value))
            }

            /// Sets the [`flip_time` field](CarState#structfield.flip_time) to the default value.
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn flip_time_as_default(self) -> CarStateBuilder<(T0, T1, T2, T3, T4, T5, T6, T7, ::planus::DefaultValue)> {
                self.flip_time(::planus::DefaultValue)
            }
        }

        impl<T0, T1, T2, T3, T4, T5, T6, T7, T8> CarStateBuilder<(T0, T1, T2, T3, T4, T5, T6, T7, T8)> {
            /// Setter for the [`is_flipping` field](CarState#structfield.is_flipping).
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn is_flipping<T9>(self, value: T9) -> CarStateBuilder<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)>
            where
                T9: ::planus::WriteAsDefault<bool, bool>,
            {
                let (v0, v1, v2, v3, v4, v5, v6, v7, v8) = self.0;
                CarStateBuilder((v0, v1, v2, v3, v4, v5, v6, v7, v8, value))
            }

            /// Sets the [`is_flipping` field](CarState#structfield.is_flipping) to the default value.
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn is_flipping_as_default(
                self,
            ) -> CarStateBuilder<(T0, T1, T2, T3, T4, T5, T6, T7, T8, ::planus::DefaultValue)> {
                self.is_flipping(::planus::DefaultValue)
            }
        }

        impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> CarStateBuilder<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)> {
            /// Setter for the [`is_jumping` field](CarState#structfield.is_jumping).
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn is_jumping<T10>(self, value: T10) -> CarStateBuilder<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)>
            where
                T10: ::planus::WriteAsDefault<bool, bool>,
            {
                let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9) = self.0;
                CarStateBuilder((v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, value))
            }

            /// Sets the [`is_jumping` field](CarState#structfield.is_jumping) to the default value.
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn is_jumping_as_default(
                self,
            ) -> CarStateBuilder<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, ::planus::DefaultValue)> {
                self.is_jumping(::planus::DefaultValue)
            }
        }

        impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> CarStateBuilder<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)> {
            /// Setter for the [`air_time` field](CarState#structfield.air_time).
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn air_time<T11>(self, value: T11) -> CarStateBuilder<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)>
            where
                T11: ::planus::WriteAsDefault<f32, f32>,
            {
                let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10) = self.0;
                CarStateBuilder((v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, value))
            }

            /// Sets the [`air_time` field](CarState#structfield.air_time) to the default value.
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn air_time_as_default(
                self,
            ) -> CarStateBuilder<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, ::planus::DefaultValue)> {
                self.air_time(::planus::DefaultValue)
            }
        }

        impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> CarStateBuilder<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)> {
            /// Setter for the [`air_time_since_jump` field](CarState#structfield.air_time_since_jump).
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn air_time_since_jump<T12>(
                self,
                value: T12,
            ) -> CarStateBuilder<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)>
            where
                T12: ::planus::WriteAsDefault<f32, f32>,
            {
                let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11) = self.0;
                CarStateBuilder((v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, value))
            }

            /// Sets the [`air_time_since_jump` field](CarState#structfield.air_time_since_jump) to the default value.
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn air_time_since_jump_as_default(
                self,
            ) -> CarStateBuilder<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, ::planus::DefaultValue)> {
                self.air_time_since_jump(::planus::DefaultValue)
            }
        }

        impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12>
            CarStateBuilder<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)>
        {
            /// Setter for the [`boost` field](CarState#structfield.boost).
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn boost<T13>(
                self,
                value: T13,
            ) -> CarStateBuilder<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)>
            where
                T13: ::planus::WriteAsDefault<f32, f32>,
            {
                let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12) = self.0;
                CarStateBuilder((v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, value))
            }

            /// Sets the [`boost` field](CarState#structfield.boost) to the default value.
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn boost_as_default(
                self,
            ) -> CarStateBuilder<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, ::planus::DefaultValue)>
            {
                self.boost(::planus::DefaultValue)
            }
        }

        impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13>
            CarStateBuilder<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)>
        {
            /// Setter for the [`time_since_boosted` field](CarState#structfield.time_since_boosted).
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn time_since_boosted<T14>(
                self,
                value: T14,
            ) -> CarStateBuilder<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)>
            where
                T14: ::planus::WriteAsDefault<f32, f32>,
            {
                let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13) = self.0;
                CarStateBuilder((v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, value))
            }

            /// Sets the [`time_since_boosted` field](CarState#structfield.time_since_boosted) to the default value.
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn time_since_boosted_as_default(
                self,
            ) -> CarStateBuilder<(
                T0,
                T1,
                T2,
                T3,
                T4,
                T5,
                T6,
                T7,
                T8,
                T9,
                T10,
                T11,
                T12,
                T13,
                ::planus::DefaultValue,
            )> {
                self.time_since_boosted(::planus::DefaultValue)
            }
        }

        impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14>
            CarStateBuilder<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)>
        {
            /// Setter for the [`is_boosting` field](CarState#structfield.is_boosting).
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn is_boosting<T15>(
                self,
                value: T15,
            ) -> CarStateBuilder<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)>
            where
                T15: ::planus::WriteAsDefault<bool, bool>,
            {
                let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14) = self.0;
                CarStateBuilder((v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, value))
            }

            /// Sets the [`is_boosting` field](CarState#structfield.is_boosting) to the default value.
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn is_boosting_as_default(
                self,
            ) -> CarStateBuilder<(
                T0,
                T1,
                T2,
                T3,
                T4,
                T5,
                T6,
                T7,
                T8,
                T9,
                T10,
                T11,
                T12,
                T13,
                T14,
                ::planus::DefaultValue,
            )> {
                self.is_boosting(::planus::DefaultValue)
            }
        }

        impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15>
            CarStateBuilder<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)>
        {
            /// Setter for the [`boosting_time` field](CarState#structfield.boosting_time).
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn boosting_time<T16>(
                self,
                value: T16,
            ) -> CarStateBuilder<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16)>
            where
                T16: ::planus::WriteAsDefault<f32, f32>,
            {
                let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15) = self.0;
                CarStateBuilder((v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, value))
            }

            /// Sets the [`boosting_time` field](CarState#structfield.boosting_time) to the default value.
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn boosting_time_as_default(
                self,
            ) -> CarStateBuilder<(
                T0,
                T1,
                T2,
                T3,
                T4,
                T5,
                T6,
                T7,
                T8,
                T9,
                T10,
                T11,
                T12,
                T13,
                T14,
                T15,
                ::planus::DefaultValue,
            )> {
                self.boosting_time(::planus::DefaultValue)
            }
        }

        impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16>
            CarStateBuilder<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16)>
        {
            /// Setter for the [`is_supersonic` field](CarState#structfield.is_supersonic).
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn is_supersonic<T17>(
                self,
                value: T17,
            ) -> CarStateBuilder<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17)>
            where
                T17: ::planus::WriteAsDefault<bool, bool>,
            {
                let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16) = self.0;
                CarStateBuilder((
                    v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, value,
                ))
            }

            /// Sets the [`is_supersonic` field](CarState#structfield.is_supersonic) to the default value.
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn is_supersonic_as_default(
                self,
            ) -> CarStateBuilder<(
                T0,
                T1,
                T2,
                T3,
                T4,
                T5,
                T6,
                T7,
                T8,
                T9,
                T10,
                T11,
                T12,
                T13,
                T14,
                T15,
                T16,
                ::planus::DefaultValue,
            )> {
                self.is_supersonic(::planus::DefaultValue)
            }
        }

        impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17>
            CarStateBuilder<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17)>
        {
            /// Setter for the [`supersonic_time` field](CarState#structfield.supersonic_time).
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn supersonic_time<T18>(
                self,
                value: T18,
            ) -> CarStateBuilder<(
                T0,
                T1,
                T2,
                T3,
                T4,
                T5,
                T6,
                T7,
                T8,
                T9,
                T10,
                T11,
                T12,
                T13,
                T14,
                T15,
                T16,
                T17,
                T18,
            )>
            where
                T18: ::planus::WriteAsDefault<f32, f32>,
            {
                let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17) = self.0;
                CarStateBuilder((
                    v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, value,
                ))
            }

            /// Sets the [`supersonic_time` field](CarState#structfield.supersonic_time) to the default value.
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn supersonic_time_as_default(
                self,
            ) -> CarStateBuilder<(
                T0,
                T1,
                T2,
                T3,
                T4,
                T5,
                T6,
                T7,
                T8,
                T9,
                T10,
                T11,
                T12,
                T13,
                T14,
                T15,
                T16,
                T17,
                ::planus::DefaultValue,
            )> {
                self.supersonic_time(::planus::DefaultValue)
            }
        }

        impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18>
            CarStateBuilder<(
                T0,
                T1,
                T2,
                T3,
                T4,
                T5,
                T6,
                T7,
                T8,
                T9,
                T10,
                T11,
                T12,
                T13,
                T14,
                T15,
                T16,
                T17,
                T18,
            )>
        {
            /// Setter for the [`handbrake_val` field](CarState#structfield.handbrake_val).
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn handbrake_val<T19>(
                self,
                value: T19,
            ) -> CarStateBuilder<(
                T0,
                T1,
                T2,
                T3,
                T4,
                T5,
                T6,
                T7,
                T8,
                T9,
                T10,
                T11,
                T12,
                T13,
                T14,
                T15,
                T16,
                T17,
                T18,
                T19,
            )>
            where
                T19: ::planus::WriteAsDefault<f32, f32>,
            {
                let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18) = self.0;
                CarStateBuilder((
                    v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, value,
                ))
            }

            /// Sets the [`handbrake_val` field](CarState#structfield.handbrake_val) to the default value.
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn handbrake_val_as_default(
                self,
            ) -> CarStateBuilder<(
                T0,
                T1,
                T2,
                T3,
                T4,
                T5,
                T6,
                T7,
                T8,
                T9,
                T10,
                T11,
                T12,
                T13,
                T14,
                T15,
                T16,
                T17,
                T18,
                ::planus::DefaultValue,
            )> {
                self.handbrake_val(::planus::DefaultValue)
            }
        }

        impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19>
            CarStateBuilder<(
                T0,
                T1,
                T2,
                T3,
                T4,
                T5,
                T6,
                T7,
                T8,
                T9,
                T10,
                T11,
                T12,
                T13,
                T14,
                T15,
                T16,
                T17,
                T18,
                T19,
            )>
        {
            /// Setter for the [`is_auto_flipping` field](CarState#structfield.is_auto_flipping).
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn is_auto_flipping<T20>(
                self,
                value: T20,
            ) -> CarStateBuilder<(
                T0,
                T1,
                T2,
                T3,
                T4,
                T5,
                T6,
                T7,
                T8,
                T9,
                T10,
                T11,
                T12,
                T13,
                T14,
                T15,
                T16,
                T17,
                T18,
                T19,
                T20,
            )>
            where
                T20: ::planus::WriteAsDefault<bool, bool>,
            {
                let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19) = self.0;
                CarStateBuilder((
                    v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, value,
                ))
            }

            /// Sets the [`is_auto_flipping` field](CarState#structfield.is_auto_flipping) to the default value.
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn is_auto_flipping_as_default(
                self,
            ) -> CarStateBuilder<(
                T0,
                T1,
                T2,
                T3,
                T4,
                T5,
                T6,
                T7,
                T8,
                T9,
                T10,
                T11,
                T12,
                T13,
                T14,
                T15,
                T16,
                T17,
                T18,
                T19,
                ::planus::DefaultValue,
            )> {
                self.is_auto_flipping(::planus::DefaultValue)
            }
        }

        impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20>
            CarStateBuilder<(
                T0,
                T1,
                T2,
                T3,
                T4,
                T5,
                T6,
                T7,
                T8,
                T9,
                T10,
                T11,
                T12,
                T13,
                T14,
                T15,
                T16,
                T17,
                T18,
                T19,
                T20,
            )>
        {
            /// Setter for the [`auto_flip_timer` field](CarState#structfield.auto_flip_timer).
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn auto_flip_timer<T21>(
                self,
                value: T21,
            ) -> CarStateBuilder<(
                T0,
                T1,
                T2,
                T3,
                T4,
                T5,
                T6,
                T7,
                T8,
                T9,
                T10,
                T11,
                T12,
                T13,
                T14,
                T15,
                T16,
                T17,
                T18,
                T19,
                T20,
                T21,
            )>
            where
                T21: ::planus::WriteAsDefault<f32, f32>,
            {
                let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20) = self.0;
                CarStateBuilder((
                    v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, value,
                ))
            }

            /// Sets the [`auto_flip_timer` field](CarState#structfield.auto_flip_timer) to the default value.
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn auto_flip_timer_as_default(
                self,
            ) -> CarStateBuilder<(
                T0,
                T1,
                T2,
                T3,
                T4,
                T5,
                T6,
                T7,
                T8,
                T9,
                T10,
                T11,
                T12,
                T13,
                T14,
                T15,
                T16,
                T17,
                T18,
                T19,
                T20,
                ::planus::DefaultValue,
            )> {
                self.auto_flip_timer(::planus::DefaultValue)
            }
        }

        impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21>
            CarStateBuilder<(
                T0,
                T1,
                T2,
                T3,
                T4,
                T5,
                T6,
                T7,
                T8,
                T9,
                T10,
                T11,
                T12,
                T13,
                T14,
                T15,
                T16,
                T17,
                T18,
                T19,
                T20,
                T21,
            )>
        {
            /// Setter for the [`auto_flip_torque_scale` field](CarState#structfield.auto_flip_torque_scale).
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn auto_flip_torque_scale<T22>(
                self,
                value: T22,
            ) -> CarStateBuilder<(
                T0,
                T1,
                T2,
                T3,
                T4,
                T5,
                T6,
                T7,
                T8,
                T9,
                T10,
                T11,
                T12,
                T13,
                T14,
                T15,
                T16,
                T17,
                T18,
                T19,
                T20,
                T21,
                T22,
            )>
            where
                T22: ::planus::WriteAsDefault<f32, f32>,
            {
                let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21) =
                    self.0;
                CarStateBuilder((
                    v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21,
                    value,
                ))
            }

            /// Sets the [`auto_flip_torque_scale` field](CarState#structfield.auto_flip_torque_scale) to the default value.
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn auto_flip_torque_scale_as_default(
                self,
            ) -> CarStateBuilder<(
                T0,
                T1,
                T2,
                T3,
                T4,
                T5,
                T6,
                T7,
                T8,
                T9,
                T10,
                T11,
                T12,
                T13,
                T14,
                T15,
                T16,
                T17,
                T18,
                T19,
                T20,
                T21,
                ::planus::DefaultValue,
            )> {
                self.auto_flip_torque_scale(::planus::DefaultValue)
            }
        }

        impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22>
            CarStateBuilder<(
                T0,
                T1,
                T2,
                T3,
                T4,
                T5,
                T6,
                T7,
                T8,
                T9,
                T10,
                T11,
                T12,
                T13,
                T14,
                T15,
                T16,
                T17,
                T18,
                T19,
                T20,
                T21,
                T22,
            )>
        {
            /// Setter for the [`world_contact_normal` field](CarState#structfield.world_contact_normal).
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn world_contact_normal<T23>(
                self,
                value: T23,
            ) -> CarStateBuilder<(
                T0,
                T1,
                T2,
                T3,
                T4,
                T5,
                T6,
                T7,
                T8,
                T9,
                T10,
                T11,
                T12,
                T13,
                T14,
                T15,
                T16,
                T17,
                T18,
                T19,
                T20,
                T21,
                T22,
                T23,
            )>
            where
                T23: ::planus::WriteAsOptional<self::Vec3>,
            {
                let (
                    v0,
                    v1,
                    v2,
                    v3,
                    v4,
                    v5,
                    v6,
                    v7,
                    v8,
                    v9,
                    v10,
                    v11,
                    v12,
                    v13,
                    v14,
                    v15,
                    v16,
                    v17,
                    v18,
                    v19,
                    v20,
                    v21,
                    v22,
                ) = self.0;
                CarStateBuilder((
                    v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22,
                    value,
                ))
            }

            /// Sets the [`world_contact_normal` field](CarState#structfield.world_contact_normal) to null.
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn world_contact_normal_as_null(
                self,
            ) -> CarStateBuilder<(
                T0,
                T1,
                T2,
                T3,
                T4,
                T5,
                T6,
                T7,
                T8,
                T9,
                T10,
                T11,
                T12,
                T13,
                T14,
                T15,
                T16,
                T17,
                T18,
                T19,
                T20,
                T21,
                T22,
                (),
            )> {
                self.world_contact_normal(())
            }
        }

        impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23>
            CarStateBuilder<(
                T0,
                T1,
                T2,
                T3,
                T4,
                T5,
                T6,
                T7,
                T8,
                T9,
                T10,
                T11,
                T12,
                T13,
                T14,
                T15,
                T16,
                T17,
                T18,
                T19,
                T20,
                T21,
                T22,
                T23,
            )>
        {
            /// Setter for the [`car_contact` field](CarState#structfield.car_contact).
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn car_contact<T24>(
                self,
                value: T24,
            ) -> CarStateBuilder<(
                T0,
                T1,
                T2,
                T3,
                T4,
                T5,
                T6,
                T7,
                T8,
                T9,
                T10,
                T11,
                T12,
                T13,
                T14,
                T15,
                T16,
                T17,
                T18,
                T19,
                T20,
                T21,
                T22,
                T23,
                T24,
            )>
            where
                T24: ::planus::WriteAsOptional<::planus::Offset<self::CarContact>>,
            {
                let (
                    v0,
                    v1,
                    v2,
                    v3,
                    v4,
                    v5,
                    v6,
                    v7,
                    v8,
                    v9,
                    v10,
                    v11,
                    v12,
                    v13,
                    v14,
                    v15,
                    v16,
                    v17,
                    v18,
                    v19,
                    v20,
                    v21,
                    v22,
                    v23,
                ) = self.0;
                CarStateBuilder((
                    v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22,
                    v23, value,
                ))
            }

            /// Sets the [`car_contact` field](CarState#structfield.car_contact) to null.
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn car_contact_as_null(
                self,
            ) -> CarStateBuilder<(
                T0,
                T1,
                T2,
                T3,
                T4,
                T5,
                T6,
                T7,
                T8,
                T9,
                T10,
                T11,
                T12,
                T13,
                T14,
                T15,
                T16,
                T17,
                T18,
                T19,
                T20,
                T21,
                T22,
                T23,
                (),
            )> {
                self.car_contact(())
            }
        }

        impl<
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
            T10,
            T11,
            T12,
            T13,
            T14,
            T15,
            T16,
            T17,
            T18,
            T19,
            T20,
            T21,
            T22,
            T23,
            T24,
        >
            CarStateBuilder<(
                T0,
                T1,
                T2,
                T3,
                T4,
                T5,
                T6,
                T7,
                T8,
                T9,
                T10,
                T11,
                T12,
                T13,
                T14,
                T15,
                T16,
                T17,
                T18,
                T19,
                T20,
                T21,
                T22,
                T23,
                T24,
            )>
        {
            /// Setter for the [`is_demoed` field](CarState#structfield.is_demoed).
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn is_demoed<T25>(
                self,
                value: T25,
            ) -> CarStateBuilder<(
                T0,
                T1,
                T2,
                T3,
                T4,
                T5,
                T6,
                T7,
                T8,
                T9,
                T10,
                T11,
                T12,
                T13,
                T14,
                T15,
                T16,
                T17,
                T18,
                T19,
                T20,
                T21,
                T22,
                T23,
                T24,
                T25,
            )>
            where
                T25: ::planus::WriteAsDefault<bool, bool>,
            {
                let (
                    v0,
                    v1,
                    v2,
                    v3,
                    v4,
                    v5,
                    v6,
                    v7,
                    v8,
                    v9,
                    v10,
                    v11,
                    v12,
                    v13,
                    v14,
                    v15,
                    v16,
                    v17,
                    v18,
                    v19,
                    v20,
                    v21,
                    v22,
                    v23,
                    v24,
                ) = self.0;
                CarStateBuilder((
                    v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22,
                    v23, v24, value,
                ))
            }

            /// Sets the [`is_demoed` field](CarState#structfield.is_demoed) to the default value.
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn is_demoed_as_default(
                self,
            ) -> CarStateBuilder<(
                T0,
                T1,
                T2,
                T3,
                T4,
                T5,
                T6,
                T7,
                T8,
                T9,
                T10,
                T11,
                T12,
                T13,
                T14,
                T15,
                T16,
                T17,
                T18,
                T19,
                T20,
                T21,
                T22,
                T23,
                T24,
                ::planus::DefaultValue,
            )> {
                self.is_demoed(::planus::DefaultValue)
            }
        }

        impl<
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
            T10,
            T11,
            T12,
            T13,
            T14,
            T15,
            T16,
            T17,
            T18,
            T19,
            T20,
            T21,
            T22,
            T23,
            T24,
            T25,
        >
            CarStateBuilder<(
                T0,
                T1,
                T2,
                T3,
                T4,
                T5,
                T6,
                T7,
                T8,
                T9,
                T10,
                T11,
                T12,
                T13,
                T14,
                T15,
                T16,
                T17,
                T18,
                T19,
                T20,
                T21,
                T22,
                T23,
                T24,
                T25,
            )>
        {
            /// Setter for the [`demo_respawn_timer` field](CarState#structfield.demo_respawn_timer).
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn demo_respawn_timer<T26>(
                self,
                value: T26,
            ) -> CarStateBuilder<(
                T0,
                T1,
                T2,
                T3,
                T4,
                T5,
                T6,
                T7,
                T8,
                T9,
                T10,
                T11,
                T12,
                T13,
                T14,
                T15,
                T16,
                T17,
                T18,
                T19,
                T20,
                T21,
                T22,
                T23,
                T24,
                T25,
                T26,
            )>
            where
                T26: ::planus::WriteAsDefault<f32, f32>,
            {
                let (
                    v0,
                    v1,
                    v2,
                    v3,
                    v4,
                    v5,
                    v6,
                    v7,
                    v8,
                    v9,
                    v10,
                    v11,
                    v12,
                    v13,
                    v14,
                    v15,
                    v16,
                    v17,
                    v18,
                    v19,
                    v20,
                    v21,
                    v22,
                    v23,
                    v24,
                    v25,
                ) = self.0;
                CarStateBuilder((
                    v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22,
                    v23, v24, v25, value,
                ))
            }

            /// Sets the [`demo_respawn_timer` field](CarState#structfield.demo_respawn_timer) to the default value.
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn demo_respawn_timer_as_default(
                self,
            ) -> CarStateBuilder<(
                T0,
                T1,
                T2,
                T3,
                T4,
                T5,
                T6,
                T7,
                T8,
                T9,
                T10,
                T11,
                T12,
                T13,
                T14,
                T15,
                T16,
                T17,
                T18,
                T19,
                T20,
                T21,
                T22,
                T23,
                T24,
                T25,
                ::planus::DefaultValue,
            )> {
                self.demo_respawn_timer(::planus::DefaultValue)
            }
        }

        impl<
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
            T10,
            T11,
            T12,
            T13,
            T14,
            T15,
            T16,
            T17,
            T18,
            T19,
            T20,
            T21,
            T22,
            T23,
            T24,
            T25,
            T26,
        >
            CarStateBuilder<(
                T0,
                T1,
                T2,
                T3,
                T4,
                T5,
                T6,
                T7,
                T8,
                T9,
                T10,
                T11,
                T12,
                T13,
                T14,
                T15,
                T16,
                T17,
                T18,
                T19,
                T20,
                T21,
                T22,
                T23,
                T24,
                T25,
                T26,
            )>
        {
            /// Setter for the [`ball_hit_info` field](CarState#structfield.ball_hit_info).
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn ball_hit_info<T27>(
                self,
                value: T27,
            ) -> CarStateBuilder<(
                T0,
                T1,
                T2,
                T3,
                T4,
                T5,
                T6,
                T7,
                T8,
                T9,
                T10,
                T11,
                T12,
                T13,
                T14,
                T15,
                T16,
                T17,
                T18,
                T19,
                T20,
                T21,
                T22,
                T23,
                T24,
                T25,
                T26,
                T27,
            )>
            where
                T27: ::planus::WriteAsOptional<::planus::Offset<self::BallHitInfo>>,
            {
                let (
                    v0,
                    v1,
                    v2,
                    v3,
                    v4,
                    v5,
                    v6,
                    v7,
                    v8,
                    v9,
                    v10,
                    v11,
                    v12,
                    v13,
                    v14,
                    v15,
                    v16,
                    v17,
                    v18,
                    v19,
                    v20,
                    v21,
                    v22,
                    v23,
                    v24,
                    v25,
                    v26,
                ) = self.0;
                CarStateBuilder((
                    v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22,
                    v23, v24, v25, v26, value,
                ))
            }

            /// Sets the [`ball_hit_info` field](CarState#structfield.ball_hit_info) to null.
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn ball_hit_info_as_null(
                self,
            ) -> CarStateBuilder<(
                T0,
                T1,
                T2,
                T3,
                T4,
                T5,
                T6,
                T7,
                T8,
                T9,
                T10,
                T11,
                T12,
                T13,
                T14,
                T15,
                T16,
                T17,
                T18,
                T19,
                T20,
                T21,
                T22,
                T23,
                T24,
                T25,
                T26,
                (),
            )> {
                self.ball_hit_info(())
            }
        }

        impl<
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
            T10,
            T11,
            T12,
            T13,
            T14,
            T15,
            T16,
            T17,
            T18,
            T19,
            T20,
            T21,
            T22,
            T23,
            T24,
            T25,
            T26,
            T27,
        >
            CarStateBuilder<(
                T0,
                T1,
                T2,
                T3,
                T4,
                T5,
                T6,
                T7,
                T8,
                T9,
                T10,
                T11,
                T12,
                T13,
                T14,
                T15,
                T16,
                T17,
                T18,
                T19,
                T20,
                T21,
                T22,
                T23,
                T24,
                T25,
                T26,
                T27,
            )>
        {
            /// Setter for the [`last_controls` field](CarState#structfield.last_controls).
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn last_controls<T28>(
                self,
                value: T28,
            ) -> CarStateBuilder<(
                T0,
                T1,
                T2,
                T3,
                T4,
                T5,
                T6,
                T7,
                T8,
                T9,
                T10,
                T11,
                T12,
                T13,
                T14,
                T15,
                T16,
                T17,
                T18,
                T19,
                T20,
                T21,
                T22,
                T23,
                T24,
                T25,
                T26,
                T27,
                T28,
            )>
            where
                T28: ::planus::WriteAs<self::CarControls>,
            {
                let (
                    v0,
                    v1,
                    v2,
                    v3,
                    v4,
                    v5,
                    v6,
                    v7,
                    v8,
                    v9,
                    v10,
                    v11,
                    v12,
                    v13,
                    v14,
                    v15,
                    v16,
                    v17,
                    v18,
                    v19,
                    v20,
                    v21,
                    v22,
                    v23,
                    v24,
                    v25,
                    v26,
                    v27,
                ) = self.0;
                CarStateBuilder((
                    v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22,
                    v23, v24, v25, v26, v27, value,
                ))
            }
        }

        impl<
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
            T10,
            T11,
            T12,
            T13,
            T14,
            T15,
            T16,
            T17,
            T18,
            T19,
            T20,
            T21,
            T22,
            T23,
            T24,
            T25,
            T26,
            T27,
            T28,
        >
            CarStateBuilder<(
                T0,
                T1,
                T2,
                T3,
                T4,
                T5,
                T6,
                T7,
                T8,
                T9,
                T10,
                T11,
                T12,
                T13,
                T14,
                T15,
                T16,
                T17,
                T18,
                T19,
                T20,
                T21,
                T22,
                T23,
                T24,
                T25,
                T26,
                T27,
                T28,
            )>
        {
            /// Finish writing the builder to get an [Offset](::planus::Offset) to a serialized [CarState].
            #[inline]
            pub fn finish(self, builder: &mut ::planus::Builder) -> ::planus::Offset<CarState>
            where
                Self: ::planus::WriteAsOffset<CarState>,
            {
                ::planus::WriteAsOffset::prepare(&self, builder)
            }
        }

        impl<
            T0: ::planus::WriteAs<self::PhysState>,
            T1: ::planus::WriteAsDefault<bool, bool>,
            T2: ::planus::WriteAs<self::WheelsWithContact>,
            T3: ::planus::WriteAsDefault<bool, bool>,
            T4: ::planus::WriteAsDefault<bool, bool>,
            T5: ::planus::WriteAsDefault<bool, bool>,
            T6: ::planus::WriteAs<self::Vec3>,
            T7: ::planus::WriteAsDefault<f32, f32>,
            T8: ::planus::WriteAsDefault<f32, f32>,
            T9: ::planus::WriteAsDefault<bool, bool>,
            T10: ::planus::WriteAsDefault<bool, bool>,
            T11: ::planus::WriteAsDefault<f32, f32>,
            T12: ::planus::WriteAsDefault<f32, f32>,
            T13: ::planus::WriteAsDefault<f32, f32>,
            T14: ::planus::WriteAsDefault<f32, f32>,
            T15: ::planus::WriteAsDefault<bool, bool>,
            T16: ::planus::WriteAsDefault<f32, f32>,
            T17: ::planus::WriteAsDefault<bool, bool>,
            T18: ::planus::WriteAsDefault<f32, f32>,
            T19: ::planus::WriteAsDefault<f32, f32>,
            T20: ::planus::WriteAsDefault<bool, bool>,
            T21: ::planus::WriteAsDefault<f32, f32>,
            T22: ::planus::WriteAsDefault<f32, f32>,
            T23: ::planus::WriteAsOptional<self::Vec3>,
            T24: ::planus::WriteAsOptional<::planus::Offset<self::CarContact>>,
            T25: ::planus::WriteAsDefault<bool, bool>,
            T26: ::planus::WriteAsDefault<f32, f32>,
            T27: ::planus::WriteAsOptional<::planus::Offset<self::BallHitInfo>>,
            T28: ::planus::WriteAs<self::CarControls>,
        > ::planus::WriteAs<::planus::Offset<CarState>>
            for CarStateBuilder<(
                T0,
                T1,
                T2,
                T3,
                T4,
                T5,
                T6,
                T7,
                T8,
                T9,
                T10,
                T11,
                T12,
                T13,
                T14,
                T15,
                T16,
                T17,
                T18,
                T19,
                T20,
                T21,
                T22,
                T23,
                T24,
                T25,
                T26,
                T27,
                T28,
            )>
        {
            type Prepared = ::planus::Offset<CarState>;

            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<CarState> {
                ::planus::WriteAsOffset::prepare(self, builder)
            }
        }

        impl<
            T0: ::planus::WriteAs<self::PhysState>,
            T1: ::planus::WriteAsDefault<bool, bool>,
            T2: ::planus::WriteAs<self::WheelsWithContact>,
            T3: ::planus::WriteAsDefault<bool, bool>,
            T4: ::planus::WriteAsDefault<bool, bool>,
            T5: ::planus::WriteAsDefault<bool, bool>,
            T6: ::planus::WriteAs<self::Vec3>,
            T7: ::planus::WriteAsDefault<f32, f32>,
            T8: ::planus::WriteAsDefault<f32, f32>,
            T9: ::planus::WriteAsDefault<bool, bool>,
            T10: ::planus::WriteAsDefault<bool, bool>,
            T11: ::planus::WriteAsDefault<f32, f32>,
            T12: ::planus::WriteAsDefault<f32, f32>,
            T13: ::planus::WriteAsDefault<f32, f32>,
            T14: ::planus::WriteAsDefault<f32, f32>,
            T15: ::planus::WriteAsDefault<bool, bool>,
            T16: ::planus::WriteAsDefault<f32, f32>,
            T17: ::planus::WriteAsDefault<bool, bool>,
            T18: ::planus::WriteAsDefault<f32, f32>,
            T19: ::planus::WriteAsDefault<f32, f32>,
            T20: ::planus::WriteAsDefault<bool, bool>,
            T21: ::planus::WriteAsDefault<f32, f32>,
            T22: ::planus::WriteAsDefault<f32, f32>,
            T23: ::planus::WriteAsOptional<self::Vec3>,
            T24: ::planus::WriteAsOptional<::planus::Offset<self::CarContact>>,
            T25: ::planus::WriteAsDefault<bool, bool>,
            T26: ::planus::WriteAsDefault<f32, f32>,
            T27: ::planus::WriteAsOptional<::planus::Offset<self::BallHitInfo>>,
            T28: ::planus::WriteAs<self::CarControls>,
        > ::planus::WriteAsOptional<::planus::Offset<CarState>>
            for CarStateBuilder<(
                T0,
                T1,
                T2,
                T3,
                T4,
                T5,
                T6,
                T7,
                T8,
                T9,
                T10,
                T11,
                T12,
                T13,
                T14,
                T15,
                T16,
                T17,
                T18,
                T19,
                T20,
                T21,
                T22,
                T23,
                T24,
                T25,
                T26,
                T27,
                T28,
            )>
        {
            type Prepared = ::planus::Offset<CarState>;

            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::core::option::Option<::planus::Offset<CarState>> {
                ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
            }
        }

        impl<
            T0: ::planus::WriteAs<self::PhysState>,
            T1: ::planus::WriteAsDefault<bool, bool>,
            T2: ::planus::WriteAs<self::WheelsWithContact>,
            T3: ::planus::WriteAsDefault<bool, bool>,
            T4: ::planus::WriteAsDefault<bool, bool>,
            T5: ::planus::WriteAsDefault<bool, bool>,
            T6: ::planus::WriteAs<self::Vec3>,
            T7: ::planus::WriteAsDefault<f32, f32>,
            T8: ::planus::WriteAsDefault<f32, f32>,
            T9: ::planus::WriteAsDefault<bool, bool>,
            T10: ::planus::WriteAsDefault<bool, bool>,
            T11: ::planus::WriteAsDefault<f32, f32>,
            T12: ::planus::WriteAsDefault<f32, f32>,
            T13: ::planus::WriteAsDefault<f32, f32>,
            T14: ::planus::WriteAsDefault<f32, f32>,
            T15: ::planus::WriteAsDefault<bool, bool>,
            T16: ::planus::WriteAsDefault<f32, f32>,
            T17: ::planus::WriteAsDefault<bool, bool>,
            T18: ::planus::WriteAsDefault<f32, f32>,
            T19: ::planus::WriteAsDefault<f32, f32>,
            T20: ::planus::WriteAsDefault<bool, bool>,
            T21: ::planus::WriteAsDefault<f32, f32>,
            T22: ::planus::WriteAsDefault<f32, f32>,
            T23: ::planus::WriteAsOptional<self::Vec3>,
            T24: ::planus::WriteAsOptional<::planus::Offset<self::CarContact>>,
            T25: ::planus::WriteAsDefault<bool, bool>,
            T26: ::planus::WriteAsDefault<f32, f32>,
            T27: ::planus::WriteAsOptional<::planus::Offset<self::BallHitInfo>>,
            T28: ::planus::WriteAs<self::CarControls>,
        > ::planus::WriteAsOffset<CarState>
            for CarStateBuilder<(
                T0,
                T1,
                T2,
                T3,
                T4,
                T5,
                T6,
                T7,
                T8,
                T9,
                T10,
                T11,
                T12,
                T13,
                T14,
                T15,
                T16,
                T17,
                T18,
                T19,
                T20,
                T21,
                T22,
                T23,
                T24,
                T25,
                T26,
                T27,
                T28,
            )>
        {
            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<CarState> {
                let (
                    v0,
                    v1,
                    v2,
                    v3,
                    v4,
                    v5,
                    v6,
                    v7,
                    v8,
                    v9,
                    v10,
                    v11,
                    v12,
                    v13,
                    v14,
                    v15,
                    v16,
                    v17,
                    v18,
                    v19,
                    v20,
                    v21,
                    v22,
                    v23,
                    v24,
                    v25,
                    v26,
                    v27,
                    v28,
                ) = &self.0;
                CarState::create(
                    builder, v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20,
                    v21, v22, v23, v24, v25, v26, v27, v28,
                )
            }
        }

        /// Reference to a deserialized [CarState].
        #[derive(Copy, Clone)]
        pub struct CarStateRef<'a>(#[allow(dead_code)] ::planus::table_reader::Table<'a>);

        impl<'a> CarStateRef<'a> {
            /// Getter for the [`physics` field](CarState#structfield.physics).
            #[inline]
            pub fn physics(&self) -> ::planus::Result<self::PhysStateRef<'a>> {
                self.0.access_required(0, "CarState", "physics")
            }

            /// Getter for the [`is_on_ground` field](CarState#structfield.is_on_ground).
            #[inline]
            pub fn is_on_ground(&self) -> ::planus::Result<bool> {
                ::core::result::Result::Ok(self.0.access(1, "CarState", "is_on_ground")?.unwrap_or(false))
            }

            /// Getter for the [`wheels_with_contact` field](CarState#structfield.wheels_with_contact).
            #[inline]
            pub fn wheels_with_contact(&self) -> ::planus::Result<self::WheelsWithContactRef<'a>> {
                self.0.access_required(2, "CarState", "wheels_with_contact")
            }

            /// Getter for the [`has_jumped` field](CarState#structfield.has_jumped).
            #[inline]
            pub fn has_jumped(&self) -> ::planus::Result<bool> {
                ::core::result::Result::Ok(self.0.access(3, "CarState", "has_jumped")?.unwrap_or(false))
            }

            /// Getter for the [`has_double_jumped` field](CarState#structfield.has_double_jumped).
            #[inline]
            pub fn has_double_jumped(&self) -> ::planus::Result<bool> {
                ::core::result::Result::Ok(self.0.access(4, "CarState", "has_double_jumped")?.unwrap_or(false))
            }

            /// Getter for the [`has_flipped` field](CarState#structfield.has_flipped).
            #[inline]
            pub fn has_flipped(&self) -> ::planus::Result<bool> {
                ::core::result::Result::Ok(self.0.access(5, "CarState", "has_flipped")?.unwrap_or(false))
            }

            /// Getter for the [`flip_rel_torque` field](CarState#structfield.flip_rel_torque).
            #[inline]
            pub fn flip_rel_torque(&self) -> ::planus::Result<self::Vec3Ref<'a>> {
                self.0.access_required(6, "CarState", "flip_rel_torque")
            }

            /// Getter for the [`jump_time` field](CarState#structfield.jump_time).
            #[inline]
            pub fn jump_time(&self) -> ::planus::Result<f32> {
                ::core::result::Result::Ok(self.0.access(7, "CarState", "jump_time")?.unwrap_or(0.0))
            }

            /// Getter for the [`flip_time` field](CarState#structfield.flip_time).
            #[inline]
            pub fn flip_time(&self) -> ::planus::Result<f32> {
                ::core::result::Result::Ok(self.0.access(8, "CarState", "flip_time")?.unwrap_or(0.0))
            }

            /// Getter for the [`is_flipping` field](CarState#structfield.is_flipping).
            #[inline]
            pub fn is_flipping(&self) -> ::planus::Result<bool> {
                ::core::result::Result::Ok(self.0.access(9, "CarState", "is_flipping")?.unwrap_or(false))
            }

            /// Getter for the [`is_jumping` field](CarState#structfield.is_jumping).
            #[inline]
            pub fn is_jumping(&self) -> ::planus::Result<bool> {
                ::core::result::Result::Ok(self.0.access(10, "CarState", "is_jumping")?.unwrap_or(false))
            }

            /// Getter for the [`air_time` field](CarState#structfield.air_time).
            #[inline]
            pub fn air_time(&self) -> ::planus::Result<f32> {
                ::core::result::Result::Ok(self.0.access(11, "CarState", "air_time")?.unwrap_or(0.0))
            }

            /// Getter for the [`air_time_since_jump` field](CarState#structfield.air_time_since_jump).
            #[inline]
            pub fn air_time_since_jump(&self) -> ::planus::Result<f32> {
                ::core::result::Result::Ok(self.0.access(12, "CarState", "air_time_since_jump")?.unwrap_or(0.0))
            }

            /// Getter for the [`boost` field](CarState#structfield.boost).
            #[inline]
            pub fn boost(&self) -> ::planus::Result<f32> {
                ::core::result::Result::Ok(self.0.access(13, "CarState", "boost")?.unwrap_or(0.0))
            }

            /// Getter for the [`time_since_boosted` field](CarState#structfield.time_since_boosted).
            #[inline]
            pub fn time_since_boosted(&self) -> ::planus::Result<f32> {
                ::core::result::Result::Ok(self.0.access(14, "CarState", "time_since_boosted")?.unwrap_or(0.0))
            }

            /// Getter for the [`is_boosting` field](CarState#structfield.is_boosting).
            #[inline]
            pub fn is_boosting(&self) -> ::planus::Result<bool> {
                ::core::result::Result::Ok(self.0.access(15, "CarState", "is_boosting")?.unwrap_or(false))
            }

            /// Getter for the [`boosting_time` field](CarState#structfield.boosting_time).
            #[inline]
            pub fn boosting_time(&self) -> ::planus::Result<f32> {
                ::core::result::Result::Ok(self.0.access(16, "CarState", "boosting_time")?.unwrap_or(0.0))
            }

            /// Getter for the [`is_supersonic` field](CarState#structfield.is_supersonic).
            #[inline]
            pub fn is_supersonic(&self) -> ::planus::Result<bool> {
                ::core::result::Result::Ok(self.0.access(17, "CarState", "is_supersonic")?.unwrap_or(false))
            }

            /// Getter for the [`supersonic_time` field](CarState#structfield.supersonic_time).
            #[inline]
            pub fn supersonic_time(&self) -> ::planus::Result<f32> {
                ::core::result::Result::Ok(self.0.access(18, "CarState", "supersonic_time")?.unwrap_or(0.0))
            }

            /// Getter for the [`handbrake_val` field](CarState#structfield.handbrake_val).
            #[inline]
            pub fn handbrake_val(&self) -> ::planus::Result<f32> {
                ::core::result::Result::Ok(self.0.access(19, "CarState", "handbrake_val")?.unwrap_or(0.0))
            }

            /// Getter for the [`is_auto_flipping` field](CarState#structfield.is_auto_flipping).
            #[inline]
            pub fn is_auto_flipping(&self) -> ::planus::Result<bool> {
                ::core::result::Result::Ok(self.0.access(20, "CarState", "is_auto_flipping")?.unwrap_or(false))
            }

            /// Getter for the [`auto_flip_timer` field](CarState#structfield.auto_flip_timer).
            #[inline]
            pub fn auto_flip_timer(&self) -> ::planus::Result<f32> {
                ::core::result::Result::Ok(self.0.access(21, "CarState", "auto_flip_timer")?.unwrap_or(0.0))
            }

            /// Getter for the [`auto_flip_torque_scale` field](CarState#structfield.auto_flip_torque_scale).
            #[inline]
            pub fn auto_flip_torque_scale(&self) -> ::planus::Result<f32> {
                ::core::result::Result::Ok(self.0.access(22, "CarState", "auto_flip_torque_scale")?.unwrap_or(0.0))
            }

            /// Getter for the [`world_contact_normal` field](CarState#structfield.world_contact_normal).
            #[inline]
            pub fn world_contact_normal(&self) -> ::planus::Result<::core::option::Option<self::Vec3Ref<'a>>> {
                self.0.access(23, "CarState", "world_contact_normal")
            }

            /// Getter for the [`car_contact` field](CarState#structfield.car_contact).
            #[inline]
            pub fn car_contact(&self) -> ::planus::Result<::core::option::Option<self::CarContactRef<'a>>> {
                self.0.access(24, "CarState", "car_contact")
            }

            /// Getter for the [`is_demoed` field](CarState#structfield.is_demoed).
            #[inline]
            pub fn is_demoed(&self) -> ::planus::Result<bool> {
                ::core::result::Result::Ok(self.0.access(25, "CarState", "is_demoed")?.unwrap_or(false))
            }

            /// Getter for the [`demo_respawn_timer` field](CarState#structfield.demo_respawn_timer).
            #[inline]
            pub fn demo_respawn_timer(&self) -> ::planus::Result<f32> {
                ::core::result::Result::Ok(self.0.access(26, "CarState", "demo_respawn_timer")?.unwrap_or(0.0))
            }

            /// Getter for the [`ball_hit_info` field](CarState#structfield.ball_hit_info).
            #[inline]
            pub fn ball_hit_info(&self) -> ::planus::Result<::core::option::Option<self::BallHitInfoRef<'a>>> {
                self.0.access(27, "CarState", "ball_hit_info")
            }

            /// Getter for the [`last_controls` field](CarState#structfield.last_controls).
            #[inline]
            pub fn last_controls(&self) -> ::planus::Result<self::CarControlsRef<'a>> {
                self.0.access_required(28, "CarState", "last_controls")
            }
        }

        impl<'a> ::core::fmt::Debug for CarStateRef<'a> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                let mut f = f.debug_struct("CarStateRef");
                f.field("physics", &self.physics());
                f.field("is_on_ground", &self.is_on_ground());
                f.field("wheels_with_contact", &self.wheels_with_contact());
                f.field("has_jumped", &self.has_jumped());
                f.field("has_double_jumped", &self.has_double_jumped());
                f.field("has_flipped", &self.has_flipped());
                f.field("flip_rel_torque", &self.flip_rel_torque());
                f.field("jump_time", &self.jump_time());
                f.field("flip_time", &self.flip_time());
                f.field("is_flipping", &self.is_flipping());
                f.field("is_jumping", &self.is_jumping());
                f.field("air_time", &self.air_time());
                f.field("air_time_since_jump", &self.air_time_since_jump());
                f.field("boost", &self.boost());
                f.field("time_since_boosted", &self.time_since_boosted());
                f.field("is_boosting", &self.is_boosting());
                f.field("boosting_time", &self.boosting_time());
                f.field("is_supersonic", &self.is_supersonic());
                f.field("supersonic_time", &self.supersonic_time());
                f.field("handbrake_val", &self.handbrake_val());
                f.field("is_auto_flipping", &self.is_auto_flipping());
                f.field("auto_flip_timer", &self.auto_flip_timer());
                f.field("auto_flip_torque_scale", &self.auto_flip_torque_scale());
                if let ::core::option::Option::Some(field_world_contact_normal) = self.world_contact_normal().transpose() {
                    f.field("world_contact_normal", &field_world_contact_normal);
                }
                if let ::core::option::Option::Some(field_car_contact) = self.car_contact().transpose() {
                    f.field("car_contact", &field_car_contact);
                }
                f.field("is_demoed", &self.is_demoed());
                f.field("demo_respawn_timer", &self.demo_respawn_timer());
                if let ::core::option::Option::Some(field_ball_hit_info) = self.ball_hit_info().transpose() {
                    f.field("ball_hit_info", &field_ball_hit_info);
                }
                f.field("last_controls", &self.last_controls());
                f.finish()
            }
        }

        impl<'a> ::core::convert::TryFrom<CarStateRef<'a>> for CarState {
            type Error = ::planus::Error;

            #[allow(unreachable_code)]
            fn try_from(value: CarStateRef<'a>) -> ::planus::Result<Self> {
                ::core::result::Result::Ok(Self {
                    physics: ::core::convert::Into::into(value.physics()?),
                    is_on_ground: ::core::convert::TryInto::try_into(value.is_on_ground()?)?,
                    wheels_with_contact: ::core::convert::Into::into(value.wheels_with_contact()?),
                    has_jumped: ::core::convert::TryInto::try_into(value.has_jumped()?)?,
                    has_double_jumped: ::core::convert::TryInto::try_into(value.has_double_jumped()?)?,
                    has_flipped: ::core::convert::TryInto::try_into(value.has_flipped()?)?,
                    flip_rel_torque: ::core::convert::Into::into(value.flip_rel_torque()?),
                    jump_time: ::core::convert::TryInto::try_into(value.jump_time()?)?,
                    flip_time: ::core::convert::TryInto::try_into(value.flip_time()?)?,
                    is_flipping: ::core::convert::TryInto::try_into(value.is_flipping()?)?,
                    is_jumping: ::core::convert::TryInto::try_into(value.is_jumping()?)?,
                    air_time: ::core::convert::TryInto::try_into(value.air_time()?)?,
                    air_time_since_jump: ::core::convert::TryInto::try_into(value.air_time_since_jump()?)?,
                    boost: ::core::convert::TryInto::try_into(value.boost()?)?,
                    time_since_boosted: ::core::convert::TryInto::try_into(value.time_since_boosted()?)?,
                    is_boosting: ::core::convert::TryInto::try_into(value.is_boosting()?)?,
                    boosting_time: ::core::convert::TryInto::try_into(value.boosting_time()?)?,
                    is_supersonic: ::core::convert::TryInto::try_into(value.is_supersonic()?)?,
                    supersonic_time: ::core::convert::TryInto::try_into(value.supersonic_time()?)?,
                    handbrake_val: ::core::convert::TryInto::try_into(value.handbrake_val()?)?,
                    is_auto_flipping: ::core::convert::TryInto::try_into(value.is_auto_flipping()?)?,
                    auto_flip_timer: ::core::convert::TryInto::try_into(value.auto_flip_timer()?)?,
                    auto_flip_torque_scale: ::core::convert::TryInto::try_into(value.auto_flip_torque_scale()?)?,
                    world_contact_normal: value.world_contact_normal()?.map(::core::convert::Into::into),
                    car_contact: if let ::core::option::Option::Some(car_contact) = value.car_contact()? {
                        ::core::option::Option::Some(::planus::alloc::boxed::Box::new(::core::convert::TryInto::try_into(
                            car_contact,
                        )?))
                    } else {
                        ::core::option::Option::None
                    },
                    is_demoed: ::core::convert::TryInto::try_into(value.is_demoed()?)?,
                    demo_respawn_timer: ::core::convert::TryInto::try_into(value.demo_respawn_timer()?)?,
                    ball_hit_info: if let ::core::option::Option::Some(ball_hit_info) = value.ball_hit_info()? {
                        ::core::option::Option::Some(::planus::alloc::boxed::Box::new(::core::convert::TryInto::try_into(
                            ball_hit_info,
                        )?))
                    } else {
                        ::core::option::Option::None
                    },
                    last_controls: ::core::convert::Into::into(value.last_controls()?),
                })
            }
        }

        impl<'a> ::planus::TableRead<'a> for CarStateRef<'a> {
            #[inline]
            fn from_buffer(
                buffer: ::planus::SliceWithStartOffset<'a>,
                offset: usize,
            ) -> ::core::result::Result<Self, ::planus::errors::ErrorKind> {
                ::core::result::Result::Ok(Self(::planus::table_reader::Table::from_buffer(buffer, offset)?))
            }
        }

        impl<'a> ::planus::VectorReadInner<'a> for CarStateRef<'a> {
            type Error = ::planus::Error;
            const STRIDE: usize = 4;

            unsafe fn from_buffer(buffer: ::planus::SliceWithStartOffset<'a>, offset: usize) -> ::planus::Result<Self> {
                ::planus::TableRead::from_buffer(buffer, offset)
                    .map_err(|error_kind| error_kind.with_error_location("[CarStateRef]", "get", buffer.offset_from_start))
            }
        }

        /// # Safety
        /// The planus compiler generates implementations that initialize
        /// the bytes in `write_values`.
        unsafe impl ::planus::VectorWrite<::planus::Offset<CarState>> for CarState {
            type Value = ::planus::Offset<CarState>;
            const STRIDE: usize = 4;
            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> Self::Value {
                ::planus::WriteAs::prepare(self, builder)
            }

            #[inline]
            unsafe fn write_values(
                values: &[::planus::Offset<CarState>],
                bytes: *mut ::core::mem::MaybeUninit<u8>,
                buffer_position: u32,
            ) {
                let bytes = bytes as *mut [::core::mem::MaybeUninit<u8>; 4];
                for (i, v) in ::core::iter::Iterator::enumerate(values.iter()) {
                    ::planus::WriteAsPrimitive::write(
                        v,
                        ::planus::Cursor::new(unsafe { &mut *bytes.add(i) }),
                        buffer_position - (Self::STRIDE * i) as u32,
                    );
                }
            }
        }

        impl<'a> ::planus::ReadAsRoot<'a> for CarStateRef<'a> {
            fn read_as_root(slice: &'a [u8]) -> ::planus::Result<Self> {
                ::planus::TableRead::from_buffer(
                    ::planus::SliceWithStartOffset {
                        buffer: slice,
                        offset_from_start: 0,
                    },
                    0,
                )
                .map_err(|error_kind| error_kind.with_error_location("[CarStateRef]", "read_as_root", 0))
            }
        }

        /// The table `CarInfo` in the namespace `rocketsim`
        ///
        /// Generated from these locations:
        /// * Table `CarInfo` in the file `spec/game_state.fbs:165`
        #[derive(Clone, Debug, PartialEq, PartialOrd, ::serde::Serialize, ::serde::Deserialize)]
        pub struct CarInfo {
            /// The field `id` in the table `CarInfo`
            pub id: u64,
            /// The field `team` in the table `CarInfo`
            pub team: self::Team,
            /// The field `state` in the table `CarInfo`
            pub state: ::planus::alloc::boxed::Box<self::CarState>,
            /// The field `config` in the table `CarInfo`
            pub config: self::CarConfig,
        }

        #[allow(clippy::derivable_impls)]
        impl ::core::default::Default for CarInfo {
            fn default() -> Self {
                Self {
                    id: 0,
                    team: self::Team::Blue,
                    state: ::core::default::Default::default(),
                    config: ::core::default::Default::default(),
                }
            }
        }

        impl CarInfo {
            /// Creates a [CarInfoBuilder] for serializing an instance of this table.
            #[inline]
            pub fn builder() -> CarInfoBuilder<()> {
                CarInfoBuilder(())
            }

            #[allow(clippy::too_many_arguments)]
            pub fn create(
                builder: &mut ::planus::Builder,
                field_id: impl ::planus::WriteAsDefault<u64, u64>,
                field_team: impl ::planus::WriteAsDefault<self::Team, self::Team>,
                field_state: impl ::planus::WriteAs<::planus::Offset<self::CarState>>,
                field_config: impl ::planus::WriteAs<self::CarConfig>,
            ) -> ::planus::Offset<Self> {
                let prepared_id = field_id.prepare(builder, &0);
                let prepared_team = field_team.prepare(builder, &self::Team::Blue);
                let prepared_state = field_state.prepare(builder);
                let prepared_config = field_config.prepare(builder);

                let mut table_writer: ::planus::table_writer::TableWriter<12> = ::core::default::Default::default();
                if prepared_id.is_some() {
                    table_writer.write_entry::<u64>(0);
                }
                table_writer.write_entry::<::planus::Offset<self::CarState>>(2);
                table_writer.write_entry::<self::CarConfig>(3);
                if prepared_team.is_some() {
                    table_writer.write_entry::<self::Team>(1);
                }

                unsafe {
                    table_writer.finish(builder, |object_writer| {
                        if let ::core::option::Option::Some(prepared_id) = prepared_id {
                            object_writer.write::<_, _, 8>(&prepared_id);
                        }
                        object_writer.write::<_, _, 4>(&prepared_state);
                        object_writer.write::<_, _, 72>(&prepared_config);
                        if let ::core::option::Option::Some(prepared_team) = prepared_team {
                            object_writer.write::<_, _, 1>(&prepared_team);
                        }
                    });
                }
                builder.current_offset()
            }
        }

        impl ::planus::WriteAs<::planus::Offset<CarInfo>> for CarInfo {
            type Prepared = ::planus::Offset<Self>;

            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<CarInfo> {
                ::planus::WriteAsOffset::prepare(self, builder)
            }
        }

        impl ::planus::WriteAsOptional<::planus::Offset<CarInfo>> for CarInfo {
            type Prepared = ::planus::Offset<Self>;

            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::core::option::Option<::planus::Offset<CarInfo>> {
                ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
            }
        }

        impl ::planus::WriteAsOffset<CarInfo> for CarInfo {
            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<CarInfo> {
                CarInfo::create(builder, self.id, self.team, &self.state, self.config)
            }
        }

        /// Builder for serializing an instance of the [CarInfo] type.
        ///
        /// Can be created using the [CarInfo::builder] method.
        #[derive(Debug)]
        #[must_use]
        pub struct CarInfoBuilder<State>(State);

        impl CarInfoBuilder<()> {
            /// Setter for the [`id` field](CarInfo#structfield.id).
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn id<T0>(self, value: T0) -> CarInfoBuilder<(T0,)>
            where
                T0: ::planus::WriteAsDefault<u64, u64>,
            {
                CarInfoBuilder((value,))
            }

            /// Sets the [`id` field](CarInfo#structfield.id) to the default value.
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn id_as_default(self) -> CarInfoBuilder<(::planus::DefaultValue,)> {
                self.id(::planus::DefaultValue)
            }
        }

        impl<T0> CarInfoBuilder<(T0,)> {
            /// Setter for the [`team` field](CarInfo#structfield.team).
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn team<T1>(self, value: T1) -> CarInfoBuilder<(T0, T1)>
            where
                T1: ::planus::WriteAsDefault<self::Team, self::Team>,
            {
                let (v0,) = self.0;
                CarInfoBuilder((v0, value))
            }

            /// Sets the [`team` field](CarInfo#structfield.team) to the default value.
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn team_as_default(self) -> CarInfoBuilder<(T0, ::planus::DefaultValue)> {
                self.team(::planus::DefaultValue)
            }
        }

        impl<T0, T1> CarInfoBuilder<(T0, T1)> {
            /// Setter for the [`state` field](CarInfo#structfield.state).
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn state<T2>(self, value: T2) -> CarInfoBuilder<(T0, T1, T2)>
            where
                T2: ::planus::WriteAs<::planus::Offset<self::CarState>>,
            {
                let (v0, v1) = self.0;
                CarInfoBuilder((v0, v1, value))
            }
        }

        impl<T0, T1, T2> CarInfoBuilder<(T0, T1, T2)> {
            /// Setter for the [`config` field](CarInfo#structfield.config).
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn config<T3>(self, value: T3) -> CarInfoBuilder<(T0, T1, T2, T3)>
            where
                T3: ::planus::WriteAs<self::CarConfig>,
            {
                let (v0, v1, v2) = self.0;
                CarInfoBuilder((v0, v1, v2, value))
            }
        }

        impl<T0, T1, T2, T3> CarInfoBuilder<(T0, T1, T2, T3)> {
            /// Finish writing the builder to get an [Offset](::planus::Offset) to a serialized [CarInfo].
            #[inline]
            pub fn finish(self, builder: &mut ::planus::Builder) -> ::planus::Offset<CarInfo>
            where
                Self: ::planus::WriteAsOffset<CarInfo>,
            {
                ::planus::WriteAsOffset::prepare(&self, builder)
            }
        }

        impl<
            T0: ::planus::WriteAsDefault<u64, u64>,
            T1: ::planus::WriteAsDefault<self::Team, self::Team>,
            T2: ::planus::WriteAs<::planus::Offset<self::CarState>>,
            T3: ::planus::WriteAs<self::CarConfig>,
        > ::planus::WriteAs<::planus::Offset<CarInfo>> for CarInfoBuilder<(T0, T1, T2, T3)>
        {
            type Prepared = ::planus::Offset<CarInfo>;

            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<CarInfo> {
                ::planus::WriteAsOffset::prepare(self, builder)
            }
        }

        impl<
            T0: ::planus::WriteAsDefault<u64, u64>,
            T1: ::planus::WriteAsDefault<self::Team, self::Team>,
            T2: ::planus::WriteAs<::planus::Offset<self::CarState>>,
            T3: ::planus::WriteAs<self::CarConfig>,
        > ::planus::WriteAsOptional<::planus::Offset<CarInfo>> for CarInfoBuilder<(T0, T1, T2, T3)>
        {
            type Prepared = ::planus::Offset<CarInfo>;

            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::core::option::Option<::planus::Offset<CarInfo>> {
                ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
            }
        }

        impl<
            T0: ::planus::WriteAsDefault<u64, u64>,
            T1: ::planus::WriteAsDefault<self::Team, self::Team>,
            T2: ::planus::WriteAs<::planus::Offset<self::CarState>>,
            T3: ::planus::WriteAs<self::CarConfig>,
        > ::planus::WriteAsOffset<CarInfo> for CarInfoBuilder<(T0, T1, T2, T3)>
        {
            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<CarInfo> {
                let (v0, v1, v2, v3) = &self.0;
                CarInfo::create(builder, v0, v1, v2, v3)
            }
        }

        /// Reference to a deserialized [CarInfo].
        #[derive(Copy, Clone)]
        pub struct CarInfoRef<'a>(#[allow(dead_code)] ::planus::table_reader::Table<'a>);

        impl<'a> CarInfoRef<'a> {
            /// Getter for the [`id` field](CarInfo#structfield.id).
            #[inline]
            pub fn id(&self) -> ::planus::Result<u64> {
                ::core::result::Result::Ok(self.0.access(0, "CarInfo", "id")?.unwrap_or(0))
            }

            /// Getter for the [`team` field](CarInfo#structfield.team).
            #[inline]
            pub fn team(&self) -> ::planus::Result<self::Team> {
                ::core::result::Result::Ok(self.0.access(1, "CarInfo", "team")?.unwrap_or(self::Team::Blue))
            }

            /// Getter for the [`state` field](CarInfo#structfield.state).
            #[inline]
            pub fn state(&self) -> ::planus::Result<self::CarStateRef<'a>> {
                self.0.access_required(2, "CarInfo", "state")
            }

            /// Getter for the [`config` field](CarInfo#structfield.config).
            #[inline]
            pub fn config(&self) -> ::planus::Result<self::CarConfigRef<'a>> {
                self.0.access_required(3, "CarInfo", "config")
            }
        }

        impl<'a> ::core::fmt::Debug for CarInfoRef<'a> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                let mut f = f.debug_struct("CarInfoRef");
                f.field("id", &self.id());
                f.field("team", &self.team());
                f.field("state", &self.state());
                f.field("config", &self.config());
                f.finish()
            }
        }

        impl<'a> ::core::convert::TryFrom<CarInfoRef<'a>> for CarInfo {
            type Error = ::planus::Error;

            #[allow(unreachable_code)]
            fn try_from(value: CarInfoRef<'a>) -> ::planus::Result<Self> {
                ::core::result::Result::Ok(Self {
                    id: ::core::convert::TryInto::try_into(value.id()?)?,
                    team: ::core::convert::TryInto::try_into(value.team()?)?,
                    state: ::planus::alloc::boxed::Box::new(::core::convert::TryInto::try_into(value.state()?)?),
                    config: ::core::convert::Into::into(value.config()?),
                })
            }
        }

        impl<'a> ::planus::TableRead<'a> for CarInfoRef<'a> {
            #[inline]
            fn from_buffer(
                buffer: ::planus::SliceWithStartOffset<'a>,
                offset: usize,
            ) -> ::core::result::Result<Self, ::planus::errors::ErrorKind> {
                ::core::result::Result::Ok(Self(::planus::table_reader::Table::from_buffer(buffer, offset)?))
            }
        }

        impl<'a> ::planus::VectorReadInner<'a> for CarInfoRef<'a> {
            type Error = ::planus::Error;
            const STRIDE: usize = 4;

            unsafe fn from_buffer(buffer: ::planus::SliceWithStartOffset<'a>, offset: usize) -> ::planus::Result<Self> {
                ::planus::TableRead::from_buffer(buffer, offset)
                    .map_err(|error_kind| error_kind.with_error_location("[CarInfoRef]", "get", buffer.offset_from_start))
            }
        }

        /// # Safety
        /// The planus compiler generates implementations that initialize
        /// the bytes in `write_values`.
        unsafe impl ::planus::VectorWrite<::planus::Offset<CarInfo>> for CarInfo {
            type Value = ::planus::Offset<CarInfo>;
            const STRIDE: usize = 4;
            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> Self::Value {
                ::planus::WriteAs::prepare(self, builder)
            }

            #[inline]
            unsafe fn write_values(
                values: &[::planus::Offset<CarInfo>],
                bytes: *mut ::core::mem::MaybeUninit<u8>,
                buffer_position: u32,
            ) {
                let bytes = bytes as *mut [::core::mem::MaybeUninit<u8>; 4];
                for (i, v) in ::core::iter::Iterator::enumerate(values.iter()) {
                    ::planus::WriteAsPrimitive::write(
                        v,
                        ::planus::Cursor::new(unsafe { &mut *bytes.add(i) }),
                        buffer_position - (Self::STRIDE * i) as u32,
                    );
                }
            }
        }

        impl<'a> ::planus::ReadAsRoot<'a> for CarInfoRef<'a> {
            fn read_as_root(slice: &'a [u8]) -> ::planus::Result<Self> {
                ::planus::TableRead::from_buffer(
                    ::planus::SliceWithStartOffset {
                        buffer: slice,
                        offset_from_start: 0,
                    },
                    0,
                )
                .map_err(|error_kind| error_kind.with_error_location("[CarInfoRef]", "read_as_root", 0))
            }
        }

        /// The table `GameState` in the namespace `rocketsim`
        ///
        /// Generated from these locations:
        /// * Table `GameState` in the file `spec/game_state.fbs:172`
        #[derive(Clone, Debug, PartialEq, PartialOrd, ::serde::Serialize, ::serde::Deserialize)]
        pub struct GameState {
            /// The field `tick_rate` in the table `GameState`
            pub tick_rate: f32,
            /// The field `tick_count` in the table `GameState`
            pub tick_count: u64,
            /// The field `game_mode` in the table `GameState`
            pub game_mode: self::GameMode,
            /// The field `cars` in the table `GameState`
            pub cars: ::core::option::Option<::planus::alloc::vec::Vec<self::CarInfo>>,
            /// The field `ball` in the table `GameState`
            pub ball: self::BallState,
            /// The field `pads` in the table `GameState`
            pub pads: ::core::option::Option<::planus::alloc::vec::Vec<self::BoostPadInfo>>,
            /// The field `tiles` in the table `GameState`
            pub tiles: ::core::option::Option<::planus::alloc::boxed::Box<self::DropshotTilesByTeam>>,
        }

        #[allow(clippy::derivable_impls)]
        impl ::core::default::Default for GameState {
            fn default() -> Self {
                Self {
                    tick_rate: 0.0,
                    tick_count: 0,
                    game_mode: self::GameMode::Soccar,
                    cars: ::core::default::Default::default(),
                    ball: ::core::default::Default::default(),
                    pads: ::core::default::Default::default(),
                    tiles: ::core::default::Default::default(),
                }
            }
        }

        impl GameState {
            /// Creates a [GameStateBuilder] for serializing an instance of this table.
            #[inline]
            pub fn builder() -> GameStateBuilder<()> {
                GameStateBuilder(())
            }

            #[allow(clippy::too_many_arguments)]
            pub fn create(
                builder: &mut ::planus::Builder,
                field_tick_rate: impl ::planus::WriteAsDefault<f32, f32>,
                field_tick_count: impl ::planus::WriteAsDefault<u64, u64>,
                field_game_mode: impl ::planus::WriteAsDefault<self::GameMode, self::GameMode>,
                field_cars: impl ::planus::WriteAsOptional<::planus::Offset<[::planus::Offset<self::CarInfo>]>>,
                field_ball: impl ::planus::WriteAs<self::BallState>,
                field_pads: impl ::planus::WriteAsOptional<::planus::Offset<[::planus::Offset<self::BoostPadInfo>]>>,
                field_tiles: impl ::planus::WriteAsOptional<::planus::Offset<self::DropshotTilesByTeam>>,
            ) -> ::planus::Offset<Self> {
                let prepared_tick_rate = field_tick_rate.prepare(builder, &0.0);
                let prepared_tick_count = field_tick_count.prepare(builder, &0);
                let prepared_game_mode = field_game_mode.prepare(builder, &self::GameMode::Soccar);
                let prepared_cars = field_cars.prepare(builder);
                let prepared_ball = field_ball.prepare(builder);
                let prepared_pads = field_pads.prepare(builder);
                let prepared_tiles = field_tiles.prepare(builder);

                let mut table_writer: ::planus::table_writer::TableWriter<18> = ::core::default::Default::default();
                if prepared_tick_count.is_some() {
                    table_writer.write_entry::<u64>(1);
                }
                table_writer.write_entry::<self::BallState>(4);
                if prepared_tick_rate.is_some() {
                    table_writer.write_entry::<f32>(0);
                }
                if prepared_cars.is_some() {
                    table_writer.write_entry::<::planus::Offset<[::planus::Offset<self::CarInfo>]>>(3);
                }
                if prepared_pads.is_some() {
                    table_writer.write_entry::<::planus::Offset<[::planus::Offset<self::BoostPadInfo>]>>(5);
                }
                if prepared_tiles.is_some() {
                    table_writer.write_entry::<::planus::Offset<self::DropshotTilesByTeam>>(6);
                }
                if prepared_game_mode.is_some() {
                    table_writer.write_entry::<self::GameMode>(2);
                }

                unsafe {
                    table_writer.finish(builder, |object_writer| {
                        if let ::core::option::Option::Some(prepared_tick_count) = prepared_tick_count {
                            object_writer.write::<_, _, 8>(&prepared_tick_count);
                        }
                        object_writer.write::<_, _, 112>(&prepared_ball);
                        if let ::core::option::Option::Some(prepared_tick_rate) = prepared_tick_rate {
                            object_writer.write::<_, _, 4>(&prepared_tick_rate);
                        }
                        if let ::core::option::Option::Some(prepared_cars) = prepared_cars {
                            object_writer.write::<_, _, 4>(&prepared_cars);
                        }
                        if let ::core::option::Option::Some(prepared_pads) = prepared_pads {
                            object_writer.write::<_, _, 4>(&prepared_pads);
                        }
                        if let ::core::option::Option::Some(prepared_tiles) = prepared_tiles {
                            object_writer.write::<_, _, 4>(&prepared_tiles);
                        }
                        if let ::core::option::Option::Some(prepared_game_mode) = prepared_game_mode {
                            object_writer.write::<_, _, 1>(&prepared_game_mode);
                        }
                    });
                }
                builder.current_offset()
            }
        }

        impl ::planus::WriteAs<::planus::Offset<GameState>> for GameState {
            type Prepared = ::planus::Offset<Self>;

            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<GameState> {
                ::planus::WriteAsOffset::prepare(self, builder)
            }
        }

        impl ::planus::WriteAsOptional<::planus::Offset<GameState>> for GameState {
            type Prepared = ::planus::Offset<Self>;

            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::core::option::Option<::planus::Offset<GameState>> {
                ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
            }
        }

        impl ::planus::WriteAsOffset<GameState> for GameState {
            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<GameState> {
                GameState::create(
                    builder,
                    self.tick_rate,
                    self.tick_count,
                    self.game_mode,
                    &self.cars,
                    self.ball,
                    &self.pads,
                    &self.tiles,
                )
            }
        }

        /// Builder for serializing an instance of the [GameState] type.
        ///
        /// Can be created using the [GameState::builder] method.
        #[derive(Debug)]
        #[must_use]
        pub struct GameStateBuilder<State>(State);

        impl GameStateBuilder<()> {
            /// Setter for the [`tick_rate` field](GameState#structfield.tick_rate).
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn tick_rate<T0>(self, value: T0) -> GameStateBuilder<(T0,)>
            where
                T0: ::planus::WriteAsDefault<f32, f32>,
            {
                GameStateBuilder((value,))
            }

            /// Sets the [`tick_rate` field](GameState#structfield.tick_rate) to the default value.
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn tick_rate_as_default(self) -> GameStateBuilder<(::planus::DefaultValue,)> {
                self.tick_rate(::planus::DefaultValue)
            }
        }

        impl<T0> GameStateBuilder<(T0,)> {
            /// Setter for the [`tick_count` field](GameState#structfield.tick_count).
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn tick_count<T1>(self, value: T1) -> GameStateBuilder<(T0, T1)>
            where
                T1: ::planus::WriteAsDefault<u64, u64>,
            {
                let (v0,) = self.0;
                GameStateBuilder((v0, value))
            }

            /// Sets the [`tick_count` field](GameState#structfield.tick_count) to the default value.
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn tick_count_as_default(self) -> GameStateBuilder<(T0, ::planus::DefaultValue)> {
                self.tick_count(::planus::DefaultValue)
            }
        }

        impl<T0, T1> GameStateBuilder<(T0, T1)> {
            /// Setter for the [`game_mode` field](GameState#structfield.game_mode).
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn game_mode<T2>(self, value: T2) -> GameStateBuilder<(T0, T1, T2)>
            where
                T2: ::planus::WriteAsDefault<self::GameMode, self::GameMode>,
            {
                let (v0, v1) = self.0;
                GameStateBuilder((v0, v1, value))
            }

            /// Sets the [`game_mode` field](GameState#structfield.game_mode) to the default value.
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn game_mode_as_default(self) -> GameStateBuilder<(T0, T1, ::planus::DefaultValue)> {
                self.game_mode(::planus::DefaultValue)
            }
        }

        impl<T0, T1, T2> GameStateBuilder<(T0, T1, T2)> {
            /// Setter for the [`cars` field](GameState#structfield.cars).
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn cars<T3>(self, value: T3) -> GameStateBuilder<(T0, T1, T2, T3)>
            where
                T3: ::planus::WriteAsOptional<::planus::Offset<[::planus::Offset<self::CarInfo>]>>,
            {
                let (v0, v1, v2) = self.0;
                GameStateBuilder((v0, v1, v2, value))
            }

            /// Sets the [`cars` field](GameState#structfield.cars) to null.
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn cars_as_null(self) -> GameStateBuilder<(T0, T1, T2, ())> {
                self.cars(())
            }
        }

        impl<T0, T1, T2, T3> GameStateBuilder<(T0, T1, T2, T3)> {
            /// Setter for the [`ball` field](GameState#structfield.ball).
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn ball<T4>(self, value: T4) -> GameStateBuilder<(T0, T1, T2, T3, T4)>
            where
                T4: ::planus::WriteAs<self::BallState>,
            {
                let (v0, v1, v2, v3) = self.0;
                GameStateBuilder((v0, v1, v2, v3, value))
            }
        }

        impl<T0, T1, T2, T3, T4> GameStateBuilder<(T0, T1, T2, T3, T4)> {
            /// Setter for the [`pads` field](GameState#structfield.pads).
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn pads<T5>(self, value: T5) -> GameStateBuilder<(T0, T1, T2, T3, T4, T5)>
            where
                T5: ::planus::WriteAsOptional<::planus::Offset<[::planus::Offset<self::BoostPadInfo>]>>,
            {
                let (v0, v1, v2, v3, v4) = self.0;
                GameStateBuilder((v0, v1, v2, v3, v4, value))
            }

            /// Sets the [`pads` field](GameState#structfield.pads) to null.
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn pads_as_null(self) -> GameStateBuilder<(T0, T1, T2, T3, T4, ())> {
                self.pads(())
            }
        }

        impl<T0, T1, T2, T3, T4, T5> GameStateBuilder<(T0, T1, T2, T3, T4, T5)> {
            /// Setter for the [`tiles` field](GameState#structfield.tiles).
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn tiles<T6>(self, value: T6) -> GameStateBuilder<(T0, T1, T2, T3, T4, T5, T6)>
            where
                T6: ::planus::WriteAsOptional<::planus::Offset<self::DropshotTilesByTeam>>,
            {
                let (v0, v1, v2, v3, v4, v5) = self.0;
                GameStateBuilder((v0, v1, v2, v3, v4, v5, value))
            }

            /// Sets the [`tiles` field](GameState#structfield.tiles) to null.
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn tiles_as_null(self) -> GameStateBuilder<(T0, T1, T2, T3, T4, T5, ())> {
                self.tiles(())
            }
        }

        impl<T0, T1, T2, T3, T4, T5, T6> GameStateBuilder<(T0, T1, T2, T3, T4, T5, T6)> {
            /// Finish writing the builder to get an [Offset](::planus::Offset) to a serialized [GameState].
            #[inline]
            pub fn finish(self, builder: &mut ::planus::Builder) -> ::planus::Offset<GameState>
            where
                Self: ::planus::WriteAsOffset<GameState>,
            {
                ::planus::WriteAsOffset::prepare(&self, builder)
            }
        }

        impl<
            T0: ::planus::WriteAsDefault<f32, f32>,
            T1: ::planus::WriteAsDefault<u64, u64>,
            T2: ::planus::WriteAsDefault<self::GameMode, self::GameMode>,
            T3: ::planus::WriteAsOptional<::planus::Offset<[::planus::Offset<self::CarInfo>]>>,
            T4: ::planus::WriteAs<self::BallState>,
            T5: ::planus::WriteAsOptional<::planus::Offset<[::planus::Offset<self::BoostPadInfo>]>>,
            T6: ::planus::WriteAsOptional<::planus::Offset<self::DropshotTilesByTeam>>,
        > ::planus::WriteAs<::planus::Offset<GameState>> for GameStateBuilder<(T0, T1, T2, T3, T4, T5, T6)>
        {
            type Prepared = ::planus::Offset<GameState>;

            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<GameState> {
                ::planus::WriteAsOffset::prepare(self, builder)
            }
        }

        impl<
            T0: ::planus::WriteAsDefault<f32, f32>,
            T1: ::planus::WriteAsDefault<u64, u64>,
            T2: ::planus::WriteAsDefault<self::GameMode, self::GameMode>,
            T3: ::planus::WriteAsOptional<::planus::Offset<[::planus::Offset<self::CarInfo>]>>,
            T4: ::planus::WriteAs<self::BallState>,
            T5: ::planus::WriteAsOptional<::planus::Offset<[::planus::Offset<self::BoostPadInfo>]>>,
            T6: ::planus::WriteAsOptional<::planus::Offset<self::DropshotTilesByTeam>>,
        > ::planus::WriteAsOptional<::planus::Offset<GameState>> for GameStateBuilder<(T0, T1, T2, T3, T4, T5, T6)>
        {
            type Prepared = ::planus::Offset<GameState>;

            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::core::option::Option<::planus::Offset<GameState>> {
                ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
            }
        }

        impl<
            T0: ::planus::WriteAsDefault<f32, f32>,
            T1: ::planus::WriteAsDefault<u64, u64>,
            T2: ::planus::WriteAsDefault<self::GameMode, self::GameMode>,
            T3: ::planus::WriteAsOptional<::planus::Offset<[::planus::Offset<self::CarInfo>]>>,
            T4: ::planus::WriteAs<self::BallState>,
            T5: ::planus::WriteAsOptional<::planus::Offset<[::planus::Offset<self::BoostPadInfo>]>>,
            T6: ::planus::WriteAsOptional<::planus::Offset<self::DropshotTilesByTeam>>,
        > ::planus::WriteAsOffset<GameState> for GameStateBuilder<(T0, T1, T2, T3, T4, T5, T6)>
        {
            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<GameState> {
                let (v0, v1, v2, v3, v4, v5, v6) = &self.0;
                GameState::create(builder, v0, v1, v2, v3, v4, v5, v6)
            }
        }

        /// Reference to a deserialized [GameState].
        #[derive(Copy, Clone)]
        pub struct GameStateRef<'a>(#[allow(dead_code)] ::planus::table_reader::Table<'a>);

        impl<'a> GameStateRef<'a> {
            /// Getter for the [`tick_rate` field](GameState#structfield.tick_rate).
            #[inline]
            pub fn tick_rate(&self) -> ::planus::Result<f32> {
                ::core::result::Result::Ok(self.0.access(0, "GameState", "tick_rate")?.unwrap_or(0.0))
            }

            /// Getter for the [`tick_count` field](GameState#structfield.tick_count).
            #[inline]
            pub fn tick_count(&self) -> ::planus::Result<u64> {
                ::core::result::Result::Ok(self.0.access(1, "GameState", "tick_count")?.unwrap_or(0))
            }

            /// Getter for the [`game_mode` field](GameState#structfield.game_mode).
            #[inline]
            pub fn game_mode(&self) -> ::planus::Result<self::GameMode> {
                ::core::result::Result::Ok(self.0.access(2, "GameState", "game_mode")?.unwrap_or(self::GameMode::Soccar))
            }

            /// Getter for the [`cars` field](GameState#structfield.cars).
            #[inline]
            pub fn cars(
                &self,
            ) -> ::planus::Result<::core::option::Option<::planus::Vector<'a, ::planus::Result<self::CarInfoRef<'a>>>>>
            {
                self.0.access(3, "GameState", "cars")
            }

            /// Getter for the [`ball` field](GameState#structfield.ball).
            #[inline]
            pub fn ball(&self) -> ::planus::Result<self::BallStateRef<'a>> {
                self.0.access_required(4, "GameState", "ball")
            }

            /// Getter for the [`pads` field](GameState#structfield.pads).
            #[inline]
            pub fn pads(
                &self,
            ) -> ::planus::Result<::core::option::Option<::planus::Vector<'a, ::planus::Result<self::BoostPadInfoRef<'a>>>>>
            {
                self.0.access(5, "GameState", "pads")
            }

            /// Getter for the [`tiles` field](GameState#structfield.tiles).
            #[inline]
            pub fn tiles(&self) -> ::planus::Result<::core::option::Option<self::DropshotTilesByTeamRef<'a>>> {
                self.0.access(6, "GameState", "tiles")
            }
        }

        impl<'a> ::core::fmt::Debug for GameStateRef<'a> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                let mut f = f.debug_struct("GameStateRef");
                f.field("tick_rate", &self.tick_rate());
                f.field("tick_count", &self.tick_count());
                f.field("game_mode", &self.game_mode());
                if let ::core::option::Option::Some(field_cars) = self.cars().transpose() {
                    f.field("cars", &field_cars);
                }
                f.field("ball", &self.ball());
                if let ::core::option::Option::Some(field_pads) = self.pads().transpose() {
                    f.field("pads", &field_pads);
                }
                if let ::core::option::Option::Some(field_tiles) = self.tiles().transpose() {
                    f.field("tiles", &field_tiles);
                }
                f.finish()
            }
        }

        impl<'a> ::core::convert::TryFrom<GameStateRef<'a>> for GameState {
            type Error = ::planus::Error;

            #[allow(unreachable_code)]
            fn try_from(value: GameStateRef<'a>) -> ::planus::Result<Self> {
                ::core::result::Result::Ok(Self {
                    tick_rate: ::core::convert::TryInto::try_into(value.tick_rate()?)?,
                    tick_count: ::core::convert::TryInto::try_into(value.tick_count()?)?,
                    game_mode: ::core::convert::TryInto::try_into(value.game_mode()?)?,
                    cars: if let ::core::option::Option::Some(cars) = value.cars()? {
                        ::core::option::Option::Some(cars.to_vec_result()?)
                    } else {
                        ::core::option::Option::None
                    },
                    ball: ::core::convert::Into::into(value.ball()?),
                    pads: if let ::core::option::Option::Some(pads) = value.pads()? {
                        ::core::option::Option::Some(pads.to_vec_result()?)
                    } else {
                        ::core::option::Option::None
                    },
                    tiles: if let ::core::option::Option::Some(tiles) = value.tiles()? {
                        ::core::option::Option::Some(::planus::alloc::boxed::Box::new(::core::convert::TryInto::try_into(
                            tiles,
                        )?))
                    } else {
                        ::core::option::Option::None
                    },
                })
            }
        }

        impl<'a> ::planus::TableRead<'a> for GameStateRef<'a> {
            #[inline]
            fn from_buffer(
                buffer: ::planus::SliceWithStartOffset<'a>,
                offset: usize,
            ) -> ::core::result::Result<Self, ::planus::errors::ErrorKind> {
                ::core::result::Result::Ok(Self(::planus::table_reader::Table::from_buffer(buffer, offset)?))
            }
        }

        impl<'a> ::planus::VectorReadInner<'a> for GameStateRef<'a> {
            type Error = ::planus::Error;
            const STRIDE: usize = 4;

            unsafe fn from_buffer(buffer: ::planus::SliceWithStartOffset<'a>, offset: usize) -> ::planus::Result<Self> {
                ::planus::TableRead::from_buffer(buffer, offset)
                    .map_err(|error_kind| error_kind.with_error_location("[GameStateRef]", "get", buffer.offset_from_start))
            }
        }

        /// # Safety
        /// The planus compiler generates implementations that initialize
        /// the bytes in `write_values`.
        unsafe impl ::planus::VectorWrite<::planus::Offset<GameState>> for GameState {
            type Value = ::planus::Offset<GameState>;
            const STRIDE: usize = 4;
            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> Self::Value {
                ::planus::WriteAs::prepare(self, builder)
            }

            #[inline]
            unsafe fn write_values(
                values: &[::planus::Offset<GameState>],
                bytes: *mut ::core::mem::MaybeUninit<u8>,
                buffer_position: u32,
            ) {
                let bytes = bytes as *mut [::core::mem::MaybeUninit<u8>; 4];
                for (i, v) in ::core::iter::Iterator::enumerate(values.iter()) {
                    ::planus::WriteAsPrimitive::write(
                        v,
                        ::planus::Cursor::new(unsafe { &mut *bytes.add(i) }),
                        buffer_position - (Self::STRIDE * i) as u32,
                    );
                }
            }
        }

        impl<'a> ::planus::ReadAsRoot<'a> for GameStateRef<'a> {
            fn read_as_root(slice: &'a [u8]) -> ::planus::Result<Self> {
                ::planus::TableRead::from_buffer(
                    ::planus::SliceWithStartOffset {
                        buffer: slice,
                        offset_from_start: 0,
                    },
                    0,
                )
                .map_err(|error_kind| error_kind.with_error_location("[GameStateRef]", "read_as_root", 0))
            }
        }

        /// The struct `Vec2` in the namespace `rocketsim`
        ///
        /// Generated from these locations:
        /// * Struct `Vec2` in the file `spec/render.fbs:5`
        #[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Default, ::serde::Serialize, ::serde::Deserialize)]
        pub struct Vec2 {
            /// The field `x` in the struct `Vec2`
            pub x: f32,

            /// The field `y` in the struct `Vec2`
            pub y: f32,
        }

        /// # Safety
        /// The Planus compiler correctly calculates `ALIGNMENT` and `SIZE`.
        unsafe impl ::planus::Primitive for Vec2 {
            const ALIGNMENT: usize = 4;
            const SIZE: usize = 8;
        }

        #[allow(clippy::identity_op)]
        impl ::planus::WriteAsPrimitive<Vec2> for Vec2 {
            #[inline]
            fn write<const N: usize>(&self, cursor: ::planus::Cursor<'_, N>, buffer_position: u32) {
                let (cur, cursor) = cursor.split::<4, 4>();
                self.x.write(cur, buffer_position - 0);
                let (cur, cursor) = cursor.split::<4, 0>();
                self.y.write(cur, buffer_position - 4);
                cursor.finish([]);
            }
        }

        impl ::planus::WriteAsOffset<Vec2> for Vec2 {
            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<Vec2> {
                unsafe {
                    builder.write_with(8, 3, |buffer_position, bytes| {
                        let bytes = bytes.as_mut_ptr();

                        ::planus::WriteAsPrimitive::write(
                            self,
                            ::planus::Cursor::new(&mut *(bytes as *mut [::core::mem::MaybeUninit<u8>; 8])),
                            buffer_position,
                        );
                    });
                }
                builder.current_offset()
            }
        }

        impl ::planus::WriteAs<Vec2> for Vec2 {
            type Prepared = Self;
            #[inline]
            fn prepare(&self, _builder: &mut ::planus::Builder) -> Self {
                *self
            }
        }

        impl ::planus::WriteAsOptional<Vec2> for Vec2 {
            type Prepared = Self;
            #[inline]
            fn prepare(&self, _builder: &mut ::planus::Builder) -> ::core::option::Option<Self> {
                ::core::option::Option::Some(*self)
            }
        }

        /// Reference to a deserialized [Vec2].
        #[derive(Copy, Clone)]
        pub struct Vec2Ref<'a>(::planus::ArrayWithStartOffset<'a, 8>);

        impl<'a> Vec2Ref<'a> {
            /// Getter for the [`x` field](Vec2#structfield.x).
            pub fn x(&self) -> f32 {
                let buffer = self.0.advance_as_array::<4>(0).unwrap();

                f32::from_le_bytes(*buffer.as_array())
            }

            /// Getter for the [`y` field](Vec2#structfield.y).
            pub fn y(&self) -> f32 {
                let buffer = self.0.advance_as_array::<4>(4).unwrap();

                f32::from_le_bytes(*buffer.as_array())
            }
        }

        impl<'a> ::core::fmt::Debug for Vec2Ref<'a> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                let mut f = f.debug_struct("Vec2Ref");
                f.field("x", &self.x());
                f.field("y", &self.y());
                f.finish()
            }
        }

        impl<'a> ::core::convert::From<::planus::ArrayWithStartOffset<'a, 8>> for Vec2Ref<'a> {
            fn from(array: ::planus::ArrayWithStartOffset<'a, 8>) -> Self {
                Self(array)
            }
        }

        impl<'a> ::core::convert::From<Vec2Ref<'a>> for Vec2 {
            #[allow(unreachable_code)]
            fn from(value: Vec2Ref<'a>) -> Self {
                Self {
                    x: value.x(),
                    y: value.y(),
                }
            }
        }

        impl<'a, 'b> ::core::cmp::PartialEq<Vec2Ref<'a>> for Vec2Ref<'b> {
            fn eq(&self, other: &Vec2Ref<'_>) -> bool {
                self.x() == other.x() && self.y() == other.y()
            }
        }

        impl<'a, 'b> ::core::cmp::PartialOrd<Vec2Ref<'a>> for Vec2Ref<'b> {
            fn partial_cmp(&self, other: &Vec2Ref<'_>) -> ::core::option::Option<::core::cmp::Ordering> {
                match self.x().partial_cmp(&other.x()) {
                    ::core::option::Option::Some(::core::cmp::Ordering::Equal) => (),
                    o => return o,
                }

                self.y().partial_cmp(&other.y())
            }
        }

        impl<'a> ::planus::TableRead<'a> for Vec2Ref<'a> {
            #[inline]
            fn from_buffer(
                buffer: ::planus::SliceWithStartOffset<'a>,
                offset: usize,
            ) -> ::core::result::Result<Self, ::planus::errors::ErrorKind> {
                let buffer = buffer.advance_as_array::<8>(offset)?;
                ::core::result::Result::Ok(Self(buffer))
            }
        }

        impl<'a> ::planus::VectorRead<'a> for Vec2Ref<'a> {
            const STRIDE: usize = 8;

            #[inline]
            unsafe fn from_buffer(buffer: ::planus::SliceWithStartOffset<'a>, offset: usize) -> Self {
                Self(unsafe { buffer.unchecked_advance_as_array(offset) })
            }
        }

        /// # Safety
        /// The planus compiler generates implementations that initialize
        /// the bytes in `write_values`.
        unsafe impl ::planus::VectorWrite<Vec2> for Vec2 {
            const STRIDE: usize = 8;

            type Value = Vec2;

            #[inline]
            fn prepare(&self, _builder: &mut ::planus::Builder) -> Self::Value {
                *self
            }

            #[inline]
            unsafe fn write_values(values: &[Vec2], bytes: *mut ::core::mem::MaybeUninit<u8>, buffer_position: u32) {
                let bytes = bytes as *mut [::core::mem::MaybeUninit<u8>; 8];
                for (i, v) in ::core::iter::Iterator::enumerate(values.iter()) {
                    ::planus::WriteAsPrimitive::write(
                        v,
                        ::planus::Cursor::new(unsafe { &mut *bytes.add(i) }),
                        buffer_position - (8 * i) as u32,
                    );
                }
            }
        }

        /// The struct `Color` in the namespace `rocketsim`
        ///
        /// Generated from these locations:
        /// * Struct `Color` in the file `spec/render.fbs:10`
        #[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Default, ::serde::Serialize, ::serde::Deserialize)]
        pub struct Color {
            /// The field `r` in the struct `Color`
            pub r: f32,

            /// The field `g` in the struct `Color`
            pub g: f32,

            /// The field `b` in the struct `Color`
            pub b: f32,

            /// The field `a` in the struct `Color`
            pub a: f32,
        }

        /// # Safety
        /// The Planus compiler correctly calculates `ALIGNMENT` and `SIZE`.
        unsafe impl ::planus::Primitive for Color {
            const ALIGNMENT: usize = 4;
            const SIZE: usize = 16;
        }

        #[allow(clippy::identity_op)]
        impl ::planus::WriteAsPrimitive<Color> for Color {
            #[inline]
            fn write<const N: usize>(&self, cursor: ::planus::Cursor<'_, N>, buffer_position: u32) {
                let (cur, cursor) = cursor.split::<4, 12>();
                self.r.write(cur, buffer_position - 0);
                let (cur, cursor) = cursor.split::<4, 8>();
                self.g.write(cur, buffer_position - 4);
                let (cur, cursor) = cursor.split::<4, 4>();
                self.b.write(cur, buffer_position - 8);
                let (cur, cursor) = cursor.split::<4, 0>();
                self.a.write(cur, buffer_position - 12);
                cursor.finish([]);
            }
        }

        impl ::planus::WriteAsOffset<Color> for Color {
            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<Color> {
                unsafe {
                    builder.write_with(16, 3, |buffer_position, bytes| {
                        let bytes = bytes.as_mut_ptr();

                        ::planus::WriteAsPrimitive::write(
                            self,
                            ::planus::Cursor::new(&mut *(bytes as *mut [::core::mem::MaybeUninit<u8>; 16])),
                            buffer_position,
                        );
                    });
                }
                builder.current_offset()
            }
        }

        impl ::planus::WriteAs<Color> for Color {
            type Prepared = Self;
            #[inline]
            fn prepare(&self, _builder: &mut ::planus::Builder) -> Self {
                *self
            }
        }

        impl ::planus::WriteAsOptional<Color> for Color {
            type Prepared = Self;
            #[inline]
            fn prepare(&self, _builder: &mut ::planus::Builder) -> ::core::option::Option<Self> {
                ::core::option::Option::Some(*self)
            }
        }

        /// Reference to a deserialized [Color].
        #[derive(Copy, Clone)]
        pub struct ColorRef<'a>(::planus::ArrayWithStartOffset<'a, 16>);

        impl<'a> ColorRef<'a> {
            /// Getter for the [`r` field](Color#structfield.r).
            pub fn r(&self) -> f32 {
                let buffer = self.0.advance_as_array::<4>(0).unwrap();

                f32::from_le_bytes(*buffer.as_array())
            }

            /// Getter for the [`g` field](Color#structfield.g).
            pub fn g(&self) -> f32 {
                let buffer = self.0.advance_as_array::<4>(4).unwrap();

                f32::from_le_bytes(*buffer.as_array())
            }

            /// Getter for the [`b` field](Color#structfield.b).
            pub fn b(&self) -> f32 {
                let buffer = self.0.advance_as_array::<4>(8).unwrap();

                f32::from_le_bytes(*buffer.as_array())
            }

            /// Getter for the [`a` field](Color#structfield.a).
            pub fn a(&self) -> f32 {
                let buffer = self.0.advance_as_array::<4>(12).unwrap();

                f32::from_le_bytes(*buffer.as_array())
            }
        }

        impl<'a> ::core::fmt::Debug for ColorRef<'a> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                let mut f = f.debug_struct("ColorRef");
                f.field("r", &self.r());
                f.field("g", &self.g());
                f.field("b", &self.b());
                f.field("a", &self.a());
                f.finish()
            }
        }

        impl<'a> ::core::convert::From<::planus::ArrayWithStartOffset<'a, 16>> for ColorRef<'a> {
            fn from(array: ::planus::ArrayWithStartOffset<'a, 16>) -> Self {
                Self(array)
            }
        }

        impl<'a> ::core::convert::From<ColorRef<'a>> for Color {
            #[allow(unreachable_code)]
            fn from(value: ColorRef<'a>) -> Self {
                Self {
                    r: value.r(),
                    g: value.g(),
                    b: value.b(),
                    a: value.a(),
                }
            }
        }

        impl<'a, 'b> ::core::cmp::PartialEq<ColorRef<'a>> for ColorRef<'b> {
            fn eq(&self, other: &ColorRef<'_>) -> bool {
                self.r() == other.r() && self.g() == other.g() && self.b() == other.b() && self.a() == other.a()
            }
        }

        impl<'a, 'b> ::core::cmp::PartialOrd<ColorRef<'a>> for ColorRef<'b> {
            fn partial_cmp(&self, other: &ColorRef<'_>) -> ::core::option::Option<::core::cmp::Ordering> {
                match self.r().partial_cmp(&other.r()) {
                    ::core::option::Option::Some(::core::cmp::Ordering::Equal) => (),
                    o => return o,
                }

                match self.g().partial_cmp(&other.g()) {
                    ::core::option::Option::Some(::core::cmp::Ordering::Equal) => (),
                    o => return o,
                }

                match self.b().partial_cmp(&other.b()) {
                    ::core::option::Option::Some(::core::cmp::Ordering::Equal) => (),
                    o => return o,
                }

                self.a().partial_cmp(&other.a())
            }
        }

        impl<'a> ::planus::TableRead<'a> for ColorRef<'a> {
            #[inline]
            fn from_buffer(
                buffer: ::planus::SliceWithStartOffset<'a>,
                offset: usize,
            ) -> ::core::result::Result<Self, ::planus::errors::ErrorKind> {
                let buffer = buffer.advance_as_array::<16>(offset)?;
                ::core::result::Result::Ok(Self(buffer))
            }
        }

        impl<'a> ::planus::VectorRead<'a> for ColorRef<'a> {
            const STRIDE: usize = 16;

            #[inline]
            unsafe fn from_buffer(buffer: ::planus::SliceWithStartOffset<'a>, offset: usize) -> Self {
                Self(unsafe { buffer.unchecked_advance_as_array(offset) })
            }
        }

        /// # Safety
        /// The planus compiler generates implementations that initialize
        /// the bytes in `write_values`.
        unsafe impl ::planus::VectorWrite<Color> for Color {
            const STRIDE: usize = 16;

            type Value = Color;

            #[inline]
            fn prepare(&self, _builder: &mut ::planus::Builder) -> Self::Value {
                *self
            }

            #[inline]
            unsafe fn write_values(values: &[Color], bytes: *mut ::core::mem::MaybeUninit<u8>, buffer_position: u32) {
                let bytes = bytes as *mut [::core::mem::MaybeUninit<u8>; 16];
                for (i, v) in ::core::iter::Iterator::enumerate(values.iter()) {
                    ::planus::WriteAsPrimitive::write(
                        v,
                        ::planus::Cursor::new(unsafe { &mut *bytes.add(i) }),
                        buffer_position - (16 * i) as u32,
                    );
                }
            }
        }

        /// The table `Line2D` in the namespace `rocketsim`
        ///
        /// Generated from these locations:
        /// * Table `Line2D` in the file `spec/render.fbs:17`
        #[derive(Clone, Debug, PartialEq, PartialOrd, ::serde::Serialize, ::serde::Deserialize)]
        pub struct Line2D {
            /// The field `start` in the table `Line2D`
            pub start: self::Vec2,
            /// The field `end` in the table `Line2D`
            pub end: self::Vec2,
            /// The field `color` in the table `Line2D`
            pub color: self::Color,
        }

        #[allow(clippy::derivable_impls)]
        impl ::core::default::Default for Line2D {
            fn default() -> Self {
                Self {
                    start: ::core::default::Default::default(),
                    end: ::core::default::Default::default(),
                    color: ::core::default::Default::default(),
                }
            }
        }

        impl Line2D {
            /// Creates a [Line2DBuilder] for serializing an instance of this table.
            #[inline]
            pub fn builder() -> Line2DBuilder<()> {
                Line2DBuilder(())
            }

            #[allow(clippy::too_many_arguments)]
            pub fn create(
                builder: &mut ::planus::Builder,
                field_start: impl ::planus::WriteAs<self::Vec2>,
                field_end: impl ::planus::WriteAs<self::Vec2>,
                field_color: impl ::planus::WriteAs<self::Color>,
            ) -> ::planus::Offset<Self> {
                let prepared_start = field_start.prepare(builder);
                let prepared_end = field_end.prepare(builder);
                let prepared_color = field_color.prepare(builder);

                let mut table_writer: ::planus::table_writer::TableWriter<10> = ::core::default::Default::default();
                table_writer.write_entry::<self::Vec2>(0);
                table_writer.write_entry::<self::Vec2>(1);
                table_writer.write_entry::<self::Color>(2);

                unsafe {
                    table_writer.finish(builder, |object_writer| {
                        object_writer.write::<_, _, 8>(&prepared_start);
                        object_writer.write::<_, _, 8>(&prepared_end);
                        object_writer.write::<_, _, 16>(&prepared_color);
                    });
                }
                builder.current_offset()
            }
        }

        impl ::planus::WriteAs<::planus::Offset<Line2D>> for Line2D {
            type Prepared = ::planus::Offset<Self>;

            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<Line2D> {
                ::planus::WriteAsOffset::prepare(self, builder)
            }
        }

        impl ::planus::WriteAsOptional<::planus::Offset<Line2D>> for Line2D {
            type Prepared = ::planus::Offset<Self>;

            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::core::option::Option<::planus::Offset<Line2D>> {
                ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
            }
        }

        impl ::planus::WriteAsOffset<Line2D> for Line2D {
            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<Line2D> {
                Line2D::create(builder, self.start, self.end, self.color)
            }
        }

        /// Builder for serializing an instance of the [Line2D] type.
        ///
        /// Can be created using the [Line2D::builder] method.
        #[derive(Debug)]
        #[must_use]
        pub struct Line2DBuilder<State>(State);

        impl Line2DBuilder<()> {
            /// Setter for the [`start` field](Line2D#structfield.start).
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn start<T0>(self, value: T0) -> Line2DBuilder<(T0,)>
            where
                T0: ::planus::WriteAs<self::Vec2>,
            {
                Line2DBuilder((value,))
            }
        }

        impl<T0> Line2DBuilder<(T0,)> {
            /// Setter for the [`end` field](Line2D#structfield.end).
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn end<T1>(self, value: T1) -> Line2DBuilder<(T0, T1)>
            where
                T1: ::planus::WriteAs<self::Vec2>,
            {
                let (v0,) = self.0;
                Line2DBuilder((v0, value))
            }
        }

        impl<T0, T1> Line2DBuilder<(T0, T1)> {
            /// Setter for the [`color` field](Line2D#structfield.color).
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn color<T2>(self, value: T2) -> Line2DBuilder<(T0, T1, T2)>
            where
                T2: ::planus::WriteAs<self::Color>,
            {
                let (v0, v1) = self.0;
                Line2DBuilder((v0, v1, value))
            }
        }

        impl<T0, T1, T2> Line2DBuilder<(T0, T1, T2)> {
            /// Finish writing the builder to get an [Offset](::planus::Offset) to a serialized [Line2D].
            #[inline]
            pub fn finish(self, builder: &mut ::planus::Builder) -> ::planus::Offset<Line2D>
            where
                Self: ::planus::WriteAsOffset<Line2D>,
            {
                ::planus::WriteAsOffset::prepare(&self, builder)
            }
        }

        impl<T0: ::planus::WriteAs<self::Vec2>, T1: ::planus::WriteAs<self::Vec2>, T2: ::planus::WriteAs<self::Color>>
            ::planus::WriteAs<::planus::Offset<Line2D>> for Line2DBuilder<(T0, T1, T2)>
        {
            type Prepared = ::planus::Offset<Line2D>;

            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<Line2D> {
                ::planus::WriteAsOffset::prepare(self, builder)
            }
        }

        impl<T0: ::planus::WriteAs<self::Vec2>, T1: ::planus::WriteAs<self::Vec2>, T2: ::planus::WriteAs<self::Color>>
            ::planus::WriteAsOptional<::planus::Offset<Line2D>> for Line2DBuilder<(T0, T1, T2)>
        {
            type Prepared = ::planus::Offset<Line2D>;

            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::core::option::Option<::planus::Offset<Line2D>> {
                ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
            }
        }

        impl<T0: ::planus::WriteAs<self::Vec2>, T1: ::planus::WriteAs<self::Vec2>, T2: ::planus::WriteAs<self::Color>>
            ::planus::WriteAsOffset<Line2D> for Line2DBuilder<(T0, T1, T2)>
        {
            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<Line2D> {
                let (v0, v1, v2) = &self.0;
                Line2D::create(builder, v0, v1, v2)
            }
        }

        /// Reference to a deserialized [Line2D].
        #[derive(Copy, Clone)]
        pub struct Line2DRef<'a>(#[allow(dead_code)] ::planus::table_reader::Table<'a>);

        impl<'a> Line2DRef<'a> {
            /// Getter for the [`start` field](Line2D#structfield.start).
            #[inline]
            pub fn start(&self) -> ::planus::Result<self::Vec2Ref<'a>> {
                self.0.access_required(0, "Line2D", "start")
            }

            /// Getter for the [`end` field](Line2D#structfield.end).
            #[inline]
            pub fn end(&self) -> ::planus::Result<self::Vec2Ref<'a>> {
                self.0.access_required(1, "Line2D", "end")
            }

            /// Getter for the [`color` field](Line2D#structfield.color).
            #[inline]
            pub fn color(&self) -> ::planus::Result<self::ColorRef<'a>> {
                self.0.access_required(2, "Line2D", "color")
            }
        }

        impl<'a> ::core::fmt::Debug for Line2DRef<'a> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                let mut f = f.debug_struct("Line2DRef");
                f.field("start", &self.start());
                f.field("end", &self.end());
                f.field("color", &self.color());
                f.finish()
            }
        }

        impl<'a> ::core::convert::TryFrom<Line2DRef<'a>> for Line2D {
            type Error = ::planus::Error;

            #[allow(unreachable_code)]
            fn try_from(value: Line2DRef<'a>) -> ::planus::Result<Self> {
                ::core::result::Result::Ok(Self {
                    start: ::core::convert::Into::into(value.start()?),
                    end: ::core::convert::Into::into(value.end()?),
                    color: ::core::convert::Into::into(value.color()?),
                })
            }
        }

        impl<'a> ::planus::TableRead<'a> for Line2DRef<'a> {
            #[inline]
            fn from_buffer(
                buffer: ::planus::SliceWithStartOffset<'a>,
                offset: usize,
            ) -> ::core::result::Result<Self, ::planus::errors::ErrorKind> {
                ::core::result::Result::Ok(Self(::planus::table_reader::Table::from_buffer(buffer, offset)?))
            }
        }

        impl<'a> ::planus::VectorReadInner<'a> for Line2DRef<'a> {
            type Error = ::planus::Error;
            const STRIDE: usize = 4;

            unsafe fn from_buffer(buffer: ::planus::SliceWithStartOffset<'a>, offset: usize) -> ::planus::Result<Self> {
                ::planus::TableRead::from_buffer(buffer, offset)
                    .map_err(|error_kind| error_kind.with_error_location("[Line2DRef]", "get", buffer.offset_from_start))
            }
        }

        /// # Safety
        /// The planus compiler generates implementations that initialize
        /// the bytes in `write_values`.
        unsafe impl ::planus::VectorWrite<::planus::Offset<Line2D>> for Line2D {
            type Value = ::planus::Offset<Line2D>;
            const STRIDE: usize = 4;
            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> Self::Value {
                ::planus::WriteAs::prepare(self, builder)
            }

            #[inline]
            unsafe fn write_values(
                values: &[::planus::Offset<Line2D>],
                bytes: *mut ::core::mem::MaybeUninit<u8>,
                buffer_position: u32,
            ) {
                let bytes = bytes as *mut [::core::mem::MaybeUninit<u8>; 4];
                for (i, v) in ::core::iter::Iterator::enumerate(values.iter()) {
                    ::planus::WriteAsPrimitive::write(
                        v,
                        ::planus::Cursor::new(unsafe { &mut *bytes.add(i) }),
                        buffer_position - (Self::STRIDE * i) as u32,
                    );
                }
            }
        }

        impl<'a> ::planus::ReadAsRoot<'a> for Line2DRef<'a> {
            fn read_as_root(slice: &'a [u8]) -> ::planus::Result<Self> {
                ::planus::TableRead::from_buffer(
                    ::planus::SliceWithStartOffset {
                        buffer: slice,
                        offset_from_start: 0,
                    },
                    0,
                )
                .map_err(|error_kind| error_kind.with_error_location("[Line2DRef]", "read_as_root", 0))
            }
        }

        /// The table `Line3D` in the namespace `rocketsim`
        ///
        /// Generated from these locations:
        /// * Table `Line3D` in the file `spec/render.fbs:23`
        #[derive(Clone, Debug, PartialEq, PartialOrd, ::serde::Serialize, ::serde::Deserialize)]
        pub struct Line3D {
            /// The field `start` in the table `Line3D`
            pub start: self::Vec3,
            /// The field `end` in the table `Line3D`
            pub end: self::Vec3,
            /// The field `color` in the table `Line3D`
            pub color: self::Color,
        }

        #[allow(clippy::derivable_impls)]
        impl ::core::default::Default for Line3D {
            fn default() -> Self {
                Self {
                    start: ::core::default::Default::default(),
                    end: ::core::default::Default::default(),
                    color: ::core::default::Default::default(),
                }
            }
        }

        impl Line3D {
            /// Creates a [Line3DBuilder] for serializing an instance of this table.
            #[inline]
            pub fn builder() -> Line3DBuilder<()> {
                Line3DBuilder(())
            }

            #[allow(clippy::too_many_arguments)]
            pub fn create(
                builder: &mut ::planus::Builder,
                field_start: impl ::planus::WriteAs<self::Vec3>,
                field_end: impl ::planus::WriteAs<self::Vec3>,
                field_color: impl ::planus::WriteAs<self::Color>,
            ) -> ::planus::Offset<Self> {
                let prepared_start = field_start.prepare(builder);
                let prepared_end = field_end.prepare(builder);
                let prepared_color = field_color.prepare(builder);

                let mut table_writer: ::planus::table_writer::TableWriter<10> = ::core::default::Default::default();
                table_writer.write_entry::<self::Vec3>(0);
                table_writer.write_entry::<self::Vec3>(1);
                table_writer.write_entry::<self::Color>(2);

                unsafe {
                    table_writer.finish(builder, |object_writer| {
                        object_writer.write::<_, _, 12>(&prepared_start);
                        object_writer.write::<_, _, 12>(&prepared_end);
                        object_writer.write::<_, _, 16>(&prepared_color);
                    });
                }
                builder.current_offset()
            }
        }

        impl ::planus::WriteAs<::planus::Offset<Line3D>> for Line3D {
            type Prepared = ::planus::Offset<Self>;

            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<Line3D> {
                ::planus::WriteAsOffset::prepare(self, builder)
            }
        }

        impl ::planus::WriteAsOptional<::planus::Offset<Line3D>> for Line3D {
            type Prepared = ::planus::Offset<Self>;

            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::core::option::Option<::planus::Offset<Line3D>> {
                ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
            }
        }

        impl ::planus::WriteAsOffset<Line3D> for Line3D {
            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<Line3D> {
                Line3D::create(builder, self.start, self.end, self.color)
            }
        }

        /// Builder for serializing an instance of the [Line3D] type.
        ///
        /// Can be created using the [Line3D::builder] method.
        #[derive(Debug)]
        #[must_use]
        pub struct Line3DBuilder<State>(State);

        impl Line3DBuilder<()> {
            /// Setter for the [`start` field](Line3D#structfield.start).
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn start<T0>(self, value: T0) -> Line3DBuilder<(T0,)>
            where
                T0: ::planus::WriteAs<self::Vec3>,
            {
                Line3DBuilder((value,))
            }
        }

        impl<T0> Line3DBuilder<(T0,)> {
            /// Setter for the [`end` field](Line3D#structfield.end).
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn end<T1>(self, value: T1) -> Line3DBuilder<(T0, T1)>
            where
                T1: ::planus::WriteAs<self::Vec3>,
            {
                let (v0,) = self.0;
                Line3DBuilder((v0, value))
            }
        }

        impl<T0, T1> Line3DBuilder<(T0, T1)> {
            /// Setter for the [`color` field](Line3D#structfield.color).
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn color<T2>(self, value: T2) -> Line3DBuilder<(T0, T1, T2)>
            where
                T2: ::planus::WriteAs<self::Color>,
            {
                let (v0, v1) = self.0;
                Line3DBuilder((v0, v1, value))
            }
        }

        impl<T0, T1, T2> Line3DBuilder<(T0, T1, T2)> {
            /// Finish writing the builder to get an [Offset](::planus::Offset) to a serialized [Line3D].
            #[inline]
            pub fn finish(self, builder: &mut ::planus::Builder) -> ::planus::Offset<Line3D>
            where
                Self: ::planus::WriteAsOffset<Line3D>,
            {
                ::planus::WriteAsOffset::prepare(&self, builder)
            }
        }

        impl<T0: ::planus::WriteAs<self::Vec3>, T1: ::planus::WriteAs<self::Vec3>, T2: ::planus::WriteAs<self::Color>>
            ::planus::WriteAs<::planus::Offset<Line3D>> for Line3DBuilder<(T0, T1, T2)>
        {
            type Prepared = ::planus::Offset<Line3D>;

            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<Line3D> {
                ::planus::WriteAsOffset::prepare(self, builder)
            }
        }

        impl<T0: ::planus::WriteAs<self::Vec3>, T1: ::planus::WriteAs<self::Vec3>, T2: ::planus::WriteAs<self::Color>>
            ::planus::WriteAsOptional<::planus::Offset<Line3D>> for Line3DBuilder<(T0, T1, T2)>
        {
            type Prepared = ::planus::Offset<Line3D>;

            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::core::option::Option<::planus::Offset<Line3D>> {
                ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
            }
        }

        impl<T0: ::planus::WriteAs<self::Vec3>, T1: ::planus::WriteAs<self::Vec3>, T2: ::planus::WriteAs<self::Color>>
            ::planus::WriteAsOffset<Line3D> for Line3DBuilder<(T0, T1, T2)>
        {
            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<Line3D> {
                let (v0, v1, v2) = &self.0;
                Line3D::create(builder, v0, v1, v2)
            }
        }

        /// Reference to a deserialized [Line3D].
        #[derive(Copy, Clone)]
        pub struct Line3DRef<'a>(#[allow(dead_code)] ::planus::table_reader::Table<'a>);

        impl<'a> Line3DRef<'a> {
            /// Getter for the [`start` field](Line3D#structfield.start).
            #[inline]
            pub fn start(&self) -> ::planus::Result<self::Vec3Ref<'a>> {
                self.0.access_required(0, "Line3D", "start")
            }

            /// Getter for the [`end` field](Line3D#structfield.end).
            #[inline]
            pub fn end(&self) -> ::planus::Result<self::Vec3Ref<'a>> {
                self.0.access_required(1, "Line3D", "end")
            }

            /// Getter for the [`color` field](Line3D#structfield.color).
            #[inline]
            pub fn color(&self) -> ::planus::Result<self::ColorRef<'a>> {
                self.0.access_required(2, "Line3D", "color")
            }
        }

        impl<'a> ::core::fmt::Debug for Line3DRef<'a> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                let mut f = f.debug_struct("Line3DRef");
                f.field("start", &self.start());
                f.field("end", &self.end());
                f.field("color", &self.color());
                f.finish()
            }
        }

        impl<'a> ::core::convert::TryFrom<Line3DRef<'a>> for Line3D {
            type Error = ::planus::Error;

            #[allow(unreachable_code)]
            fn try_from(value: Line3DRef<'a>) -> ::planus::Result<Self> {
                ::core::result::Result::Ok(Self {
                    start: ::core::convert::Into::into(value.start()?),
                    end: ::core::convert::Into::into(value.end()?),
                    color: ::core::convert::Into::into(value.color()?),
                })
            }
        }

        impl<'a> ::planus::TableRead<'a> for Line3DRef<'a> {
            #[inline]
            fn from_buffer(
                buffer: ::planus::SliceWithStartOffset<'a>,
                offset: usize,
            ) -> ::core::result::Result<Self, ::planus::errors::ErrorKind> {
                ::core::result::Result::Ok(Self(::planus::table_reader::Table::from_buffer(buffer, offset)?))
            }
        }

        impl<'a> ::planus::VectorReadInner<'a> for Line3DRef<'a> {
            type Error = ::planus::Error;
            const STRIDE: usize = 4;

            unsafe fn from_buffer(buffer: ::planus::SliceWithStartOffset<'a>, offset: usize) -> ::planus::Result<Self> {
                ::planus::TableRead::from_buffer(buffer, offset)
                    .map_err(|error_kind| error_kind.with_error_location("[Line3DRef]", "get", buffer.offset_from_start))
            }
        }

        /// # Safety
        /// The planus compiler generates implementations that initialize
        /// the bytes in `write_values`.
        unsafe impl ::planus::VectorWrite<::planus::Offset<Line3D>> for Line3D {
            type Value = ::planus::Offset<Line3D>;
            const STRIDE: usize = 4;
            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> Self::Value {
                ::planus::WriteAs::prepare(self, builder)
            }

            #[inline]
            unsafe fn write_values(
                values: &[::planus::Offset<Line3D>],
                bytes: *mut ::core::mem::MaybeUninit<u8>,
                buffer_position: u32,
            ) {
                let bytes = bytes as *mut [::core::mem::MaybeUninit<u8>; 4];
                for (i, v) in ::core::iter::Iterator::enumerate(values.iter()) {
                    ::planus::WriteAsPrimitive::write(
                        v,
                        ::planus::Cursor::new(unsafe { &mut *bytes.add(i) }),
                        buffer_position - (Self::STRIDE * i) as u32,
                    );
                }
            }
        }

        impl<'a> ::planus::ReadAsRoot<'a> for Line3DRef<'a> {
            fn read_as_root(slice: &'a [u8]) -> ::planus::Result<Self> {
                ::planus::TableRead::from_buffer(
                    ::planus::SliceWithStartOffset {
                        buffer: slice,
                        offset_from_start: 0,
                    },
                    0,
                )
                .map_err(|error_kind| error_kind.with_error_location("[Line3DRef]", "read_as_root", 0))
            }
        }

        /// The table `LineStrip` in the namespace `rocketsim`
        ///
        /// Generated from these locations:
        /// * Table `LineStrip` in the file `spec/render.fbs:29`
        #[derive(Clone, Debug, PartialEq, PartialOrd, ::serde::Serialize, ::serde::Deserialize)]
        pub struct LineStrip {
            /// The field `positions` in the table `LineStrip`
            pub positions: ::planus::alloc::vec::Vec<self::Vec3>,
            /// The field `color` in the table `LineStrip`
            pub color: self::Color,
        }

        #[allow(clippy::derivable_impls)]
        impl ::core::default::Default for LineStrip {
            fn default() -> Self {
                Self {
                    positions: ::core::default::Default::default(),
                    color: ::core::default::Default::default(),
                }
            }
        }

        impl LineStrip {
            /// Creates a [LineStripBuilder] for serializing an instance of this table.
            #[inline]
            pub fn builder() -> LineStripBuilder<()> {
                LineStripBuilder(())
            }

            #[allow(clippy::too_many_arguments)]
            pub fn create(
                builder: &mut ::planus::Builder,
                field_positions: impl ::planus::WriteAs<::planus::Offset<[self::Vec3]>>,
                field_color: impl ::planus::WriteAs<self::Color>,
            ) -> ::planus::Offset<Self> {
                let prepared_positions = field_positions.prepare(builder);
                let prepared_color = field_color.prepare(builder);

                let mut table_writer: ::planus::table_writer::TableWriter<8> = ::core::default::Default::default();
                table_writer.write_entry::<::planus::Offset<[self::Vec3]>>(0);
                table_writer.write_entry::<self::Color>(1);

                unsafe {
                    table_writer.finish(builder, |object_writer| {
                        object_writer.write::<_, _, 4>(&prepared_positions);
                        object_writer.write::<_, _, 16>(&prepared_color);
                    });
                }
                builder.current_offset()
            }
        }

        impl ::planus::WriteAs<::planus::Offset<LineStrip>> for LineStrip {
            type Prepared = ::planus::Offset<Self>;

            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<LineStrip> {
                ::planus::WriteAsOffset::prepare(self, builder)
            }
        }

        impl ::planus::WriteAsOptional<::planus::Offset<LineStrip>> for LineStrip {
            type Prepared = ::planus::Offset<Self>;

            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::core::option::Option<::planus::Offset<LineStrip>> {
                ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
            }
        }

        impl ::planus::WriteAsOffset<LineStrip> for LineStrip {
            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<LineStrip> {
                LineStrip::create(builder, &self.positions, self.color)
            }
        }

        /// Builder for serializing an instance of the [LineStrip] type.
        ///
        /// Can be created using the [LineStrip::builder] method.
        #[derive(Debug)]
        #[must_use]
        pub struct LineStripBuilder<State>(State);

        impl LineStripBuilder<()> {
            /// Setter for the [`positions` field](LineStrip#structfield.positions).
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn positions<T0>(self, value: T0) -> LineStripBuilder<(T0,)>
            where
                T0: ::planus::WriteAs<::planus::Offset<[self::Vec3]>>,
            {
                LineStripBuilder((value,))
            }
        }

        impl<T0> LineStripBuilder<(T0,)> {
            /// Setter for the [`color` field](LineStrip#structfield.color).
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn color<T1>(self, value: T1) -> LineStripBuilder<(T0, T1)>
            where
                T1: ::planus::WriteAs<self::Color>,
            {
                let (v0,) = self.0;
                LineStripBuilder((v0, value))
            }
        }

        impl<T0, T1> LineStripBuilder<(T0, T1)> {
            /// Finish writing the builder to get an [Offset](::planus::Offset) to a serialized [LineStrip].
            #[inline]
            pub fn finish(self, builder: &mut ::planus::Builder) -> ::planus::Offset<LineStrip>
            where
                Self: ::planus::WriteAsOffset<LineStrip>,
            {
                ::planus::WriteAsOffset::prepare(&self, builder)
            }
        }

        impl<T0: ::planus::WriteAs<::planus::Offset<[self::Vec3]>>, T1: ::planus::WriteAs<self::Color>>
            ::planus::WriteAs<::planus::Offset<LineStrip>> for LineStripBuilder<(T0, T1)>
        {
            type Prepared = ::planus::Offset<LineStrip>;

            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<LineStrip> {
                ::planus::WriteAsOffset::prepare(self, builder)
            }
        }

        impl<T0: ::planus::WriteAs<::planus::Offset<[self::Vec3]>>, T1: ::planus::WriteAs<self::Color>>
            ::planus::WriteAsOptional<::planus::Offset<LineStrip>> for LineStripBuilder<(T0, T1)>
        {
            type Prepared = ::planus::Offset<LineStrip>;

            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::core::option::Option<::planus::Offset<LineStrip>> {
                ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
            }
        }

        impl<T0: ::planus::WriteAs<::planus::Offset<[self::Vec3]>>, T1: ::planus::WriteAs<self::Color>>
            ::planus::WriteAsOffset<LineStrip> for LineStripBuilder<(T0, T1)>
        {
            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<LineStrip> {
                let (v0, v1) = &self.0;
                LineStrip::create(builder, v0, v1)
            }
        }

        /// Reference to a deserialized [LineStrip].
        #[derive(Copy, Clone)]
        pub struct LineStripRef<'a>(#[allow(dead_code)] ::planus::table_reader::Table<'a>);

        impl<'a> LineStripRef<'a> {
            /// Getter for the [`positions` field](LineStrip#structfield.positions).
            #[inline]
            pub fn positions(&self) -> ::planus::Result<::planus::Vector<'a, self::Vec3Ref<'a>>> {
                self.0.access_required(0, "LineStrip", "positions")
            }

            /// Getter for the [`color` field](LineStrip#structfield.color).
            #[inline]
            pub fn color(&self) -> ::planus::Result<self::ColorRef<'a>> {
                self.0.access_required(1, "LineStrip", "color")
            }
        }

        impl<'a> ::core::fmt::Debug for LineStripRef<'a> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                let mut f = f.debug_struct("LineStripRef");
                f.field("positions", &self.positions());
                f.field("color", &self.color());
                f.finish()
            }
        }

        impl<'a> ::core::convert::TryFrom<LineStripRef<'a>> for LineStrip {
            type Error = ::planus::Error;

            #[allow(unreachable_code)]
            fn try_from(value: LineStripRef<'a>) -> ::planus::Result<Self> {
                ::core::result::Result::Ok(Self {
                    positions: value.positions()?.to_vec()?,
                    color: ::core::convert::Into::into(value.color()?),
                })
            }
        }

        impl<'a> ::planus::TableRead<'a> for LineStripRef<'a> {
            #[inline]
            fn from_buffer(
                buffer: ::planus::SliceWithStartOffset<'a>,
                offset: usize,
            ) -> ::core::result::Result<Self, ::planus::errors::ErrorKind> {
                ::core::result::Result::Ok(Self(::planus::table_reader::Table::from_buffer(buffer, offset)?))
            }
        }

        impl<'a> ::planus::VectorReadInner<'a> for LineStripRef<'a> {
            type Error = ::planus::Error;
            const STRIDE: usize = 4;

            unsafe fn from_buffer(buffer: ::planus::SliceWithStartOffset<'a>, offset: usize) -> ::planus::Result<Self> {
                ::planus::TableRead::from_buffer(buffer, offset)
                    .map_err(|error_kind| error_kind.with_error_location("[LineStripRef]", "get", buffer.offset_from_start))
            }
        }

        /// # Safety
        /// The planus compiler generates implementations that initialize
        /// the bytes in `write_values`.
        unsafe impl ::planus::VectorWrite<::planus::Offset<LineStrip>> for LineStrip {
            type Value = ::planus::Offset<LineStrip>;
            const STRIDE: usize = 4;
            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> Self::Value {
                ::planus::WriteAs::prepare(self, builder)
            }

            #[inline]
            unsafe fn write_values(
                values: &[::planus::Offset<LineStrip>],
                bytes: *mut ::core::mem::MaybeUninit<u8>,
                buffer_position: u32,
            ) {
                let bytes = bytes as *mut [::core::mem::MaybeUninit<u8>; 4];
                for (i, v) in ::core::iter::Iterator::enumerate(values.iter()) {
                    ::planus::WriteAsPrimitive::write(
                        v,
                        ::planus::Cursor::new(unsafe { &mut *bytes.add(i) }),
                        buffer_position - (Self::STRIDE * i) as u32,
                    );
                }
            }
        }

        impl<'a> ::planus::ReadAsRoot<'a> for LineStripRef<'a> {
            fn read_as_root(slice: &'a [u8]) -> ::planus::Result<Self> {
                ::planus::TableRead::from_buffer(
                    ::planus::SliceWithStartOffset {
                        buffer: slice,
                        offset_from_start: 0,
                    },
                    0,
                )
                .map_err(|error_kind| error_kind.with_error_location("[LineStripRef]", "read_as_root", 0))
            }
        }

        /// The union `Render` in the namespace `rocketsim`
        ///
        /// Generated from these locations:
        /// * Union `Render` in the file `spec/render.fbs:34`
        #[derive(Clone, Debug, PartialEq, PartialOrd, ::serde::Serialize, ::serde::Deserialize)]
        pub enum Render {
            /// The variant of type `Line2D` in the union `Render`
            Line2D(::planus::alloc::boxed::Box<self::Line2D>),

            /// The variant of type `Line3D` in the union `Render`
            Line3D(::planus::alloc::boxed::Box<self::Line3D>),

            /// The variant of type `LineStrip` in the union `Render`
            LineStrip(::planus::alloc::boxed::Box<self::LineStrip>),
        }

        impl Render {
            /// Creates a [RenderBuilder] for serializing an instance of this table.
            #[inline]
            pub fn builder() -> RenderBuilder<::planus::Uninitialized> {
                RenderBuilder(::planus::Uninitialized)
            }

            #[inline]
            pub fn create_line2_d(
                builder: &mut ::planus::Builder,
                value: impl ::planus::WriteAsOffset<self::Line2D>,
            ) -> ::planus::UnionOffset<Self> {
                ::planus::UnionOffset::new(1, value.prepare(builder).downcast())
            }

            #[inline]
            pub fn create_line3_d(
                builder: &mut ::planus::Builder,
                value: impl ::planus::WriteAsOffset<self::Line3D>,
            ) -> ::planus::UnionOffset<Self> {
                ::planus::UnionOffset::new(2, value.prepare(builder).downcast())
            }

            #[inline]
            pub fn create_line_strip(
                builder: &mut ::planus::Builder,
                value: impl ::planus::WriteAsOffset<self::LineStrip>,
            ) -> ::planus::UnionOffset<Self> {
                ::planus::UnionOffset::new(3, value.prepare(builder).downcast())
            }
        }

        impl ::planus::WriteAsUnion<Render> for Render {
            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::UnionOffset<Self> {
                match self {
                    Self::Line2D(value) => Self::create_line2_d(builder, value),
                    Self::Line3D(value) => Self::create_line3_d(builder, value),
                    Self::LineStrip(value) => Self::create_line_strip(builder, value),
                }
            }
        }

        impl ::planus::WriteAsOptionalUnion<Render> for Render {
            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::core::option::Option<::planus::UnionOffset<Self>> {
                ::core::option::Option::Some(::planus::WriteAsUnion::prepare(self, builder))
            }
        }

        /// Builder for serializing an instance of the [Render] type.
        ///
        /// Can be created using the [Render::builder] method.
        #[derive(Debug)]
        #[must_use]
        pub struct RenderBuilder<T>(T);

        impl RenderBuilder<::planus::Uninitialized> {
            /// Creates an instance of the [`Line2D` variant](Render#variant.Line2D).
            #[inline]
            pub fn line2_d<T>(self, value: T) -> RenderBuilder<::planus::Initialized<1, T>>
            where
                T: ::planus::WriteAsOffset<self::Line2D>,
            {
                RenderBuilder(::planus::Initialized(value))
            }

            /// Creates an instance of the [`Line3D` variant](Render#variant.Line3D).
            #[inline]
            pub fn line3_d<T>(self, value: T) -> RenderBuilder<::planus::Initialized<2, T>>
            where
                T: ::planus::WriteAsOffset<self::Line3D>,
            {
                RenderBuilder(::planus::Initialized(value))
            }

            /// Creates an instance of the [`LineStrip` variant](Render#variant.LineStrip).
            #[inline]
            pub fn line_strip<T>(self, value: T) -> RenderBuilder<::planus::Initialized<3, T>>
            where
                T: ::planus::WriteAsOffset<self::LineStrip>,
            {
                RenderBuilder(::planus::Initialized(value))
            }
        }

        impl<const N: u8, T> RenderBuilder<::planus::Initialized<N, T>> {
            /// Finish writing the builder to get an [UnionOffset](::planus::UnionOffset) to a serialized [Render].
            #[inline]
            pub fn finish(self, builder: &mut ::planus::Builder) -> ::planus::UnionOffset<Render>
            where
                Self: ::planus::WriteAsUnion<Render>,
            {
                ::planus::WriteAsUnion::prepare(&self, builder)
            }
        }

        impl<T> ::planus::WriteAsUnion<Render> for RenderBuilder<::planus::Initialized<1, T>>
        where
            T: ::planus::WriteAsOffset<self::Line2D>,
        {
            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::UnionOffset<Render> {
                ::planus::UnionOffset::new(1, (self.0).0.prepare(builder).downcast())
            }
        }

        impl<T> ::planus::WriteAsOptionalUnion<Render> for RenderBuilder<::planus::Initialized<1, T>>
        where
            T: ::planus::WriteAsOffset<self::Line2D>,
        {
            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::core::option::Option<::planus::UnionOffset<Render>> {
                ::core::option::Option::Some(::planus::WriteAsUnion::prepare(self, builder))
            }
        }
        impl<T> ::planus::WriteAsUnion<Render> for RenderBuilder<::planus::Initialized<2, T>>
        where
            T: ::planus::WriteAsOffset<self::Line3D>,
        {
            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::UnionOffset<Render> {
                ::planus::UnionOffset::new(2, (self.0).0.prepare(builder).downcast())
            }
        }

        impl<T> ::planus::WriteAsOptionalUnion<Render> for RenderBuilder<::planus::Initialized<2, T>>
        where
            T: ::planus::WriteAsOffset<self::Line3D>,
        {
            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::core::option::Option<::planus::UnionOffset<Render>> {
                ::core::option::Option::Some(::planus::WriteAsUnion::prepare(self, builder))
            }
        }
        impl<T> ::planus::WriteAsUnion<Render> for RenderBuilder<::planus::Initialized<3, T>>
        where
            T: ::planus::WriteAsOffset<self::LineStrip>,
        {
            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::UnionOffset<Render> {
                ::planus::UnionOffset::new(3, (self.0).0.prepare(builder).downcast())
            }
        }

        impl<T> ::planus::WriteAsOptionalUnion<Render> for RenderBuilder<::planus::Initialized<3, T>>
        where
            T: ::planus::WriteAsOffset<self::LineStrip>,
        {
            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::core::option::Option<::planus::UnionOffset<Render>> {
                ::core::option::Option::Some(::planus::WriteAsUnion::prepare(self, builder))
            }
        }

        /// Reference to a deserialized [Render].
        #[derive(Copy, Clone, Debug)]
        pub enum RenderRef<'a> {
            Line2D(self::Line2DRef<'a>),
            Line3D(self::Line3DRef<'a>),
            LineStrip(self::LineStripRef<'a>),
        }

        impl<'a> ::core::convert::TryFrom<RenderRef<'a>> for Render {
            type Error = ::planus::Error;

            fn try_from(value: RenderRef<'a>) -> ::planus::Result<Self> {
                ::core::result::Result::Ok(match value {
                    RenderRef::Line2D(value) => {
                        Self::Line2D(::planus::alloc::boxed::Box::new(::core::convert::TryFrom::try_from(value)?))
                    }

                    RenderRef::Line3D(value) => {
                        Self::Line3D(::planus::alloc::boxed::Box::new(::core::convert::TryFrom::try_from(value)?))
                    }

                    RenderRef::LineStrip(value) => {
                        Self::LineStrip(::planus::alloc::boxed::Box::new(::core::convert::TryFrom::try_from(value)?))
                    }
                })
            }
        }

        impl<'a> ::planus::TableReadUnion<'a> for RenderRef<'a> {
            fn from_buffer(
                buffer: ::planus::SliceWithStartOffset<'a>,
                tag: u8,
                field_offset: usize,
            ) -> ::core::result::Result<Self, ::planus::errors::ErrorKind> {
                match tag {
                    1 => ::core::result::Result::Ok(Self::Line2D(::planus::TableRead::from_buffer(buffer, field_offset)?)),
                    2 => ::core::result::Result::Ok(Self::Line3D(::planus::TableRead::from_buffer(buffer, field_offset)?)),
                    3 => {
                        ::core::result::Result::Ok(Self::LineStrip(::planus::TableRead::from_buffer(buffer, field_offset)?))
                    }
                    _ => ::core::result::Result::Err(::planus::errors::ErrorKind::UnknownUnionTag { tag }),
                }
            }
        }

        impl<'a> ::planus::VectorReadUnion<'a> for RenderRef<'a> {
            const VECTOR_NAME: &'static str = "[RenderRef]";
        }

        /// The table `AddRender` in the namespace `rocketsim`
        ///
        /// Generated from these locations:
        /// * Table `AddRender` in the file `spec/render.fbs:40`
        #[derive(Clone, Debug, PartialEq, PartialOrd, ::serde::Serialize, ::serde::Deserialize)]
        pub struct AddRender {
            /// The field `id` in the table `AddRender`
            pub id: i32,
            /// The field `commands` in the table `AddRender`
            pub commands: ::planus::alloc::vec::Vec<self::Render>,
        }

        #[allow(clippy::derivable_impls)]
        impl ::core::default::Default for AddRender {
            fn default() -> Self {
                Self {
                    id: 0,
                    commands: ::core::default::Default::default(),
                }
            }
        }

        impl AddRender {
            /// Creates a [AddRenderBuilder] for serializing an instance of this table.
            #[inline]
            pub fn builder() -> AddRenderBuilder<()> {
                AddRenderBuilder(())
            }

            #[allow(clippy::too_many_arguments)]
            pub fn create(
                builder: &mut ::planus::Builder,
                field_id: impl ::planus::WriteAsDefault<i32, i32>,
                field_commands: impl ::planus::WriteAsUnionVector<self::Render>,
            ) -> ::planus::Offset<Self> {
                let prepared_id = field_id.prepare(builder, &0);
                let prepared_commands = field_commands.prepare(builder);

                let mut table_writer: ::planus::table_writer::TableWriter<10> = ::core::default::Default::default();
                if prepared_id.is_some() {
                    table_writer.write_entry::<i32>(0);
                }
                table_writer.write_entry::<::planus::Offset<[u8]>>(1);
                table_writer.write_entry::<::planus::Offset<[::planus::Offset<self::Render>]>>(2);

                unsafe {
                    table_writer.finish(builder, |object_writer| {
                        if let ::core::option::Option::Some(prepared_id) = prepared_id {
                            object_writer.write::<_, _, 4>(&prepared_id);
                        }
                        object_writer.write::<_, _, 4>(&prepared_commands.tags_offset());
                        object_writer.write::<_, _, 4>(&prepared_commands.values_offset());
                    });
                }
                builder.current_offset()
            }
        }

        impl ::planus::WriteAs<::planus::Offset<AddRender>> for AddRender {
            type Prepared = ::planus::Offset<Self>;

            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<AddRender> {
                ::planus::WriteAsOffset::prepare(self, builder)
            }
        }

        impl ::planus::WriteAsOptional<::planus::Offset<AddRender>> for AddRender {
            type Prepared = ::planus::Offset<Self>;

            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::core::option::Option<::planus::Offset<AddRender>> {
                ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
            }
        }

        impl ::planus::WriteAsOffset<AddRender> for AddRender {
            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<AddRender> {
                AddRender::create(builder, self.id, &self.commands)
            }
        }

        /// Builder for serializing an instance of the [AddRender] type.
        ///
        /// Can be created using the [AddRender::builder] method.
        #[derive(Debug)]
        #[must_use]
        pub struct AddRenderBuilder<State>(State);

        impl AddRenderBuilder<()> {
            /// Setter for the [`id` field](AddRender#structfield.id).
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn id<T0>(self, value: T0) -> AddRenderBuilder<(T0,)>
            where
                T0: ::planus::WriteAsDefault<i32, i32>,
            {
                AddRenderBuilder((value,))
            }

            /// Sets the [`id` field](AddRender#structfield.id) to the default value.
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn id_as_default(self) -> AddRenderBuilder<(::planus::DefaultValue,)> {
                self.id(::planus::DefaultValue)
            }
        }

        impl<T0> AddRenderBuilder<(T0,)> {
            /// Setter for the [`commands` field](AddRender#structfield.commands).
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn commands<T1>(self, value: T1) -> AddRenderBuilder<(T0, T1)>
            where
                T1: ::planus::WriteAsUnionVector<self::Render>,
            {
                let (v0,) = self.0;
                AddRenderBuilder((v0, value))
            }
        }

        impl<T0, T1> AddRenderBuilder<(T0, T1)> {
            /// Finish writing the builder to get an [Offset](::planus::Offset) to a serialized [AddRender].
            #[inline]
            pub fn finish(self, builder: &mut ::planus::Builder) -> ::planus::Offset<AddRender>
            where
                Self: ::planus::WriteAsOffset<AddRender>,
            {
                ::planus::WriteAsOffset::prepare(&self, builder)
            }
        }

        impl<T0: ::planus::WriteAsDefault<i32, i32>, T1: ::planus::WriteAsUnionVector<self::Render>>
            ::planus::WriteAs<::planus::Offset<AddRender>> for AddRenderBuilder<(T0, T1)>
        {
            type Prepared = ::planus::Offset<AddRender>;

            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<AddRender> {
                ::planus::WriteAsOffset::prepare(self, builder)
            }
        }

        impl<T0: ::planus::WriteAsDefault<i32, i32>, T1: ::planus::WriteAsUnionVector<self::Render>>
            ::planus::WriteAsOptional<::planus::Offset<AddRender>> for AddRenderBuilder<(T0, T1)>
        {
            type Prepared = ::planus::Offset<AddRender>;

            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::core::option::Option<::planus::Offset<AddRender>> {
                ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
            }
        }

        impl<T0: ::planus::WriteAsDefault<i32, i32>, T1: ::planus::WriteAsUnionVector<self::Render>>
            ::planus::WriteAsOffset<AddRender> for AddRenderBuilder<(T0, T1)>
        {
            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<AddRender> {
                let (v0, v1) = &self.0;
                AddRender::create(builder, v0, v1)
            }
        }

        /// Reference to a deserialized [AddRender].
        #[derive(Copy, Clone)]
        pub struct AddRenderRef<'a>(#[allow(dead_code)] ::planus::table_reader::Table<'a>);

        impl<'a> AddRenderRef<'a> {
            /// Getter for the [`id` field](AddRender#structfield.id).
            #[inline]
            pub fn id(&self) -> ::planus::Result<i32> {
                ::core::result::Result::Ok(self.0.access(0, "AddRender", "id")?.unwrap_or(0))
            }

            /// Getter for the [`commands` field](AddRender#structfield.commands).
            #[inline]
            pub fn commands(&self) -> ::planus::Result<::planus::UnionVector<'a, self::RenderRef<'a>>> {
                self.0.access_union_vector_required(1, "AddRender", "commands")
            }
        }

        impl<'a> ::core::fmt::Debug for AddRenderRef<'a> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                let mut f = f.debug_struct("AddRenderRef");
                f.field("id", &self.id());
                f.field("commands", &self.commands());
                f.finish()
            }
        }

        impl<'a> ::core::convert::TryFrom<AddRenderRef<'a>> for AddRender {
            type Error = ::planus::Error;

            #[allow(unreachable_code)]
            fn try_from(value: AddRenderRef<'a>) -> ::planus::Result<Self> {
                ::core::result::Result::Ok(Self {
                    id: ::core::convert::TryInto::try_into(value.id()?)?,
                    commands: value.commands()?.to_vec()?,
                })
            }
        }

        impl<'a> ::planus::TableRead<'a> for AddRenderRef<'a> {
            #[inline]
            fn from_buffer(
                buffer: ::planus::SliceWithStartOffset<'a>,
                offset: usize,
            ) -> ::core::result::Result<Self, ::planus::errors::ErrorKind> {
                ::core::result::Result::Ok(Self(::planus::table_reader::Table::from_buffer(buffer, offset)?))
            }
        }

        impl<'a> ::planus::VectorReadInner<'a> for AddRenderRef<'a> {
            type Error = ::planus::Error;
            const STRIDE: usize = 4;

            unsafe fn from_buffer(buffer: ::planus::SliceWithStartOffset<'a>, offset: usize) -> ::planus::Result<Self> {
                ::planus::TableRead::from_buffer(buffer, offset)
                    .map_err(|error_kind| error_kind.with_error_location("[AddRenderRef]", "get", buffer.offset_from_start))
            }
        }

        /// # Safety
        /// The planus compiler generates implementations that initialize
        /// the bytes in `write_values`.
        unsafe impl ::planus::VectorWrite<::planus::Offset<AddRender>> for AddRender {
            type Value = ::planus::Offset<AddRender>;
            const STRIDE: usize = 4;
            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> Self::Value {
                ::planus::WriteAs::prepare(self, builder)
            }

            #[inline]
            unsafe fn write_values(
                values: &[::planus::Offset<AddRender>],
                bytes: *mut ::core::mem::MaybeUninit<u8>,
                buffer_position: u32,
            ) {
                let bytes = bytes as *mut [::core::mem::MaybeUninit<u8>; 4];
                for (i, v) in ::core::iter::Iterator::enumerate(values.iter()) {
                    ::planus::WriteAsPrimitive::write(
                        v,
                        ::planus::Cursor::new(unsafe { &mut *bytes.add(i) }),
                        buffer_position - (Self::STRIDE * i) as u32,
                    );
                }
            }
        }

        impl<'a> ::planus::ReadAsRoot<'a> for AddRenderRef<'a> {
            fn read_as_root(slice: &'a [u8]) -> ::planus::Result<Self> {
                ::planus::TableRead::from_buffer(
                    ::planus::SliceWithStartOffset {
                        buffer: slice,
                        offset_from_start: 0,
                    },
                    0,
                )
                .map_err(|error_kind| error_kind.with_error_location("[AddRenderRef]", "read_as_root", 0))
            }
        }

        /// The table `RemoveRender` in the namespace `rocketsim`
        ///
        /// Generated from these locations:
        /// * Table `RemoveRender` in the file `spec/render.fbs:45`
        #[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, ::serde::Serialize, ::serde::Deserialize)]
        pub struct RemoveRender {
            /// The field `id` in the table `RemoveRender`
            pub id: i32,
        }

        #[allow(clippy::derivable_impls)]
        impl ::core::default::Default for RemoveRender {
            fn default() -> Self {
                Self { id: 0 }
            }
        }

        impl RemoveRender {
            /// Creates a [RemoveRenderBuilder] for serializing an instance of this table.
            #[inline]
            pub fn builder() -> RemoveRenderBuilder<()> {
                RemoveRenderBuilder(())
            }

            #[allow(clippy::too_many_arguments)]
            pub fn create(
                builder: &mut ::planus::Builder,
                field_id: impl ::planus::WriteAsDefault<i32, i32>,
            ) -> ::planus::Offset<Self> {
                let prepared_id = field_id.prepare(builder, &0);

                let mut table_writer: ::planus::table_writer::TableWriter<6> = ::core::default::Default::default();
                if prepared_id.is_some() {
                    table_writer.write_entry::<i32>(0);
                }

                unsafe {
                    table_writer.finish(builder, |object_writer| {
                        if let ::core::option::Option::Some(prepared_id) = prepared_id {
                            object_writer.write::<_, _, 4>(&prepared_id);
                        }
                    });
                }
                builder.current_offset()
            }
        }

        impl ::planus::WriteAs<::planus::Offset<RemoveRender>> for RemoveRender {
            type Prepared = ::planus::Offset<Self>;

            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<RemoveRender> {
                ::planus::WriteAsOffset::prepare(self, builder)
            }
        }

        impl ::planus::WriteAsOptional<::planus::Offset<RemoveRender>> for RemoveRender {
            type Prepared = ::planus::Offset<Self>;

            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::core::option::Option<::planus::Offset<RemoveRender>> {
                ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
            }
        }

        impl ::planus::WriteAsOffset<RemoveRender> for RemoveRender {
            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<RemoveRender> {
                RemoveRender::create(builder, self.id)
            }
        }

        /// Builder for serializing an instance of the [RemoveRender] type.
        ///
        /// Can be created using the [RemoveRender::builder] method.
        #[derive(Debug)]
        #[must_use]
        pub struct RemoveRenderBuilder<State>(State);

        impl RemoveRenderBuilder<()> {
            /// Setter for the [`id` field](RemoveRender#structfield.id).
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn id<T0>(self, value: T0) -> RemoveRenderBuilder<(T0,)>
            where
                T0: ::planus::WriteAsDefault<i32, i32>,
            {
                RemoveRenderBuilder((value,))
            }

            /// Sets the [`id` field](RemoveRender#structfield.id) to the default value.
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn id_as_default(self) -> RemoveRenderBuilder<(::planus::DefaultValue,)> {
                self.id(::planus::DefaultValue)
            }
        }

        impl<T0> RemoveRenderBuilder<(T0,)> {
            /// Finish writing the builder to get an [Offset](::planus::Offset) to a serialized [RemoveRender].
            #[inline]
            pub fn finish(self, builder: &mut ::planus::Builder) -> ::planus::Offset<RemoveRender>
            where
                Self: ::planus::WriteAsOffset<RemoveRender>,
            {
                ::planus::WriteAsOffset::prepare(&self, builder)
            }
        }

        impl<T0: ::planus::WriteAsDefault<i32, i32>> ::planus::WriteAs<::planus::Offset<RemoveRender>>
            for RemoveRenderBuilder<(T0,)>
        {
            type Prepared = ::planus::Offset<RemoveRender>;

            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<RemoveRender> {
                ::planus::WriteAsOffset::prepare(self, builder)
            }
        }

        impl<T0: ::planus::WriteAsDefault<i32, i32>> ::planus::WriteAsOptional<::planus::Offset<RemoveRender>>
            for RemoveRenderBuilder<(T0,)>
        {
            type Prepared = ::planus::Offset<RemoveRender>;

            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::core::option::Option<::planus::Offset<RemoveRender>> {
                ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
            }
        }

        impl<T0: ::planus::WriteAsDefault<i32, i32>> ::planus::WriteAsOffset<RemoveRender> for RemoveRenderBuilder<(T0,)> {
            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<RemoveRender> {
                let (v0,) = &self.0;
                RemoveRender::create(builder, v0)
            }
        }

        /// Reference to a deserialized [RemoveRender].
        #[derive(Copy, Clone)]
        pub struct RemoveRenderRef<'a>(#[allow(dead_code)] ::planus::table_reader::Table<'a>);

        impl<'a> RemoveRenderRef<'a> {
            /// Getter for the [`id` field](RemoveRender#structfield.id).
            #[inline]
            pub fn id(&self) -> ::planus::Result<i32> {
                ::core::result::Result::Ok(self.0.access(0, "RemoveRender", "id")?.unwrap_or(0))
            }
        }

        impl<'a> ::core::fmt::Debug for RemoveRenderRef<'a> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                let mut f = f.debug_struct("RemoveRenderRef");
                f.field("id", &self.id());
                f.finish()
            }
        }

        impl<'a> ::core::convert::TryFrom<RemoveRenderRef<'a>> for RemoveRender {
            type Error = ::planus::Error;

            #[allow(unreachable_code)]
            fn try_from(value: RemoveRenderRef<'a>) -> ::planus::Result<Self> {
                ::core::result::Result::Ok(Self {
                    id: ::core::convert::TryInto::try_into(value.id()?)?,
                })
            }
        }

        impl<'a> ::planus::TableRead<'a> for RemoveRenderRef<'a> {
            #[inline]
            fn from_buffer(
                buffer: ::planus::SliceWithStartOffset<'a>,
                offset: usize,
            ) -> ::core::result::Result<Self, ::planus::errors::ErrorKind> {
                ::core::result::Result::Ok(Self(::planus::table_reader::Table::from_buffer(buffer, offset)?))
            }
        }

        impl<'a> ::planus::VectorReadInner<'a> for RemoveRenderRef<'a> {
            type Error = ::planus::Error;
            const STRIDE: usize = 4;

            unsafe fn from_buffer(buffer: ::planus::SliceWithStartOffset<'a>, offset: usize) -> ::planus::Result<Self> {
                ::planus::TableRead::from_buffer(buffer, offset).map_err(|error_kind| {
                    error_kind.with_error_location("[RemoveRenderRef]", "get", buffer.offset_from_start)
                })
            }
        }

        /// # Safety
        /// The planus compiler generates implementations that initialize
        /// the bytes in `write_values`.
        unsafe impl ::planus::VectorWrite<::planus::Offset<RemoveRender>> for RemoveRender {
            type Value = ::planus::Offset<RemoveRender>;
            const STRIDE: usize = 4;
            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> Self::Value {
                ::planus::WriteAs::prepare(self, builder)
            }

            #[inline]
            unsafe fn write_values(
                values: &[::planus::Offset<RemoveRender>],
                bytes: *mut ::core::mem::MaybeUninit<u8>,
                buffer_position: u32,
            ) {
                let bytes = bytes as *mut [::core::mem::MaybeUninit<u8>; 4];
                for (i, v) in ::core::iter::Iterator::enumerate(values.iter()) {
                    ::planus::WriteAsPrimitive::write(
                        v,
                        ::planus::Cursor::new(unsafe { &mut *bytes.add(i) }),
                        buffer_position - (Self::STRIDE * i) as u32,
                    );
                }
            }
        }

        impl<'a> ::planus::ReadAsRoot<'a> for RemoveRenderRef<'a> {
            fn read_as_root(slice: &'a [u8]) -> ::planus::Result<Self> {
                ::planus::TableRead::from_buffer(
                    ::planus::SliceWithStartOffset {
                        buffer: slice,
                        offset_from_start: 0,
                    },
                    0,
                )
                .map_err(|error_kind| error_kind.with_error_location("[RemoveRenderRef]", "read_as_root", 0))
            }
        }

        /// The struct `Vec3` in the namespace `rocketsim`
        ///
        /// Generated from these locations:
        /// * Struct `Vec3` in the file `spec/common.fbs:3`
        #[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Default, ::serde::Serialize, ::serde::Deserialize)]
        pub struct Vec3 {
            /// The field `x` in the struct `Vec3`
            pub x: f32,

            /// The field `y` in the struct `Vec3`
            pub y: f32,

            /// The field `z` in the struct `Vec3`
            pub z: f32,
        }

        /// # Safety
        /// The Planus compiler correctly calculates `ALIGNMENT` and `SIZE`.
        unsafe impl ::planus::Primitive for Vec3 {
            const ALIGNMENT: usize = 4;
            const SIZE: usize = 12;
        }

        #[allow(clippy::identity_op)]
        impl ::planus::WriteAsPrimitive<Vec3> for Vec3 {
            #[inline]
            fn write<const N: usize>(&self, cursor: ::planus::Cursor<'_, N>, buffer_position: u32) {
                let (cur, cursor) = cursor.split::<4, 8>();
                self.x.write(cur, buffer_position - 0);
                let (cur, cursor) = cursor.split::<4, 4>();
                self.y.write(cur, buffer_position - 4);
                let (cur, cursor) = cursor.split::<4, 0>();
                self.z.write(cur, buffer_position - 8);
                cursor.finish([]);
            }
        }

        impl ::planus::WriteAsOffset<Vec3> for Vec3 {
            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<Vec3> {
                unsafe {
                    builder.write_with(12, 3, |buffer_position, bytes| {
                        let bytes = bytes.as_mut_ptr();

                        ::planus::WriteAsPrimitive::write(
                            self,
                            ::planus::Cursor::new(&mut *(bytes as *mut [::core::mem::MaybeUninit<u8>; 12])),
                            buffer_position,
                        );
                    });
                }
                builder.current_offset()
            }
        }

        impl ::planus::WriteAs<Vec3> for Vec3 {
            type Prepared = Self;
            #[inline]
            fn prepare(&self, _builder: &mut ::planus::Builder) -> Self {
                *self
            }
        }

        impl ::planus::WriteAsOptional<Vec3> for Vec3 {
            type Prepared = Self;
            #[inline]
            fn prepare(&self, _builder: &mut ::planus::Builder) -> ::core::option::Option<Self> {
                ::core::option::Option::Some(*self)
            }
        }

        /// Reference to a deserialized [Vec3].
        #[derive(Copy, Clone)]
        pub struct Vec3Ref<'a>(::planus::ArrayWithStartOffset<'a, 12>);

        impl<'a> Vec3Ref<'a> {
            /// Getter for the [`x` field](Vec3#structfield.x).
            pub fn x(&self) -> f32 {
                let buffer = self.0.advance_as_array::<4>(0).unwrap();

                f32::from_le_bytes(*buffer.as_array())
            }

            /// Getter for the [`y` field](Vec3#structfield.y).
            pub fn y(&self) -> f32 {
                let buffer = self.0.advance_as_array::<4>(4).unwrap();

                f32::from_le_bytes(*buffer.as_array())
            }

            /// Getter for the [`z` field](Vec3#structfield.z).
            pub fn z(&self) -> f32 {
                let buffer = self.0.advance_as_array::<4>(8).unwrap();

                f32::from_le_bytes(*buffer.as_array())
            }
        }

        impl<'a> ::core::fmt::Debug for Vec3Ref<'a> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                let mut f = f.debug_struct("Vec3Ref");
                f.field("x", &self.x());
                f.field("y", &self.y());
                f.field("z", &self.z());
                f.finish()
            }
        }

        impl<'a> ::core::convert::From<::planus::ArrayWithStartOffset<'a, 12>> for Vec3Ref<'a> {
            fn from(array: ::planus::ArrayWithStartOffset<'a, 12>) -> Self {
                Self(array)
            }
        }

        impl<'a> ::core::convert::From<Vec3Ref<'a>> for Vec3 {
            #[allow(unreachable_code)]
            fn from(value: Vec3Ref<'a>) -> Self {
                Self {
                    x: value.x(),
                    y: value.y(),
                    z: value.z(),
                }
            }
        }

        impl<'a, 'b> ::core::cmp::PartialEq<Vec3Ref<'a>> for Vec3Ref<'b> {
            fn eq(&self, other: &Vec3Ref<'_>) -> bool {
                self.x() == other.x() && self.y() == other.y() && self.z() == other.z()
            }
        }

        impl<'a, 'b> ::core::cmp::PartialOrd<Vec3Ref<'a>> for Vec3Ref<'b> {
            fn partial_cmp(&self, other: &Vec3Ref<'_>) -> ::core::option::Option<::core::cmp::Ordering> {
                match self.x().partial_cmp(&other.x()) {
                    ::core::option::Option::Some(::core::cmp::Ordering::Equal) => (),
                    o => return o,
                }

                match self.y().partial_cmp(&other.y()) {
                    ::core::option::Option::Some(::core::cmp::Ordering::Equal) => (),
                    o => return o,
                }

                self.z().partial_cmp(&other.z())
            }
        }

        impl<'a> ::planus::TableRead<'a> for Vec3Ref<'a> {
            #[inline]
            fn from_buffer(
                buffer: ::planus::SliceWithStartOffset<'a>,
                offset: usize,
            ) -> ::core::result::Result<Self, ::planus::errors::ErrorKind> {
                let buffer = buffer.advance_as_array::<12>(offset)?;
                ::core::result::Result::Ok(Self(buffer))
            }
        }

        impl<'a> ::planus::VectorRead<'a> for Vec3Ref<'a> {
            const STRIDE: usize = 12;

            #[inline]
            unsafe fn from_buffer(buffer: ::planus::SliceWithStartOffset<'a>, offset: usize) -> Self {
                Self(unsafe { buffer.unchecked_advance_as_array(offset) })
            }
        }

        /// # Safety
        /// The planus compiler generates implementations that initialize
        /// the bytes in `write_values`.
        unsafe impl ::planus::VectorWrite<Vec3> for Vec3 {
            const STRIDE: usize = 12;

            type Value = Vec3;

            #[inline]
            fn prepare(&self, _builder: &mut ::planus::Builder) -> Self::Value {
                *self
            }

            #[inline]
            unsafe fn write_values(values: &[Vec3], bytes: *mut ::core::mem::MaybeUninit<u8>, buffer_position: u32) {
                let bytes = bytes as *mut [::core::mem::MaybeUninit<u8>; 12];
                for (i, v) in ::core::iter::Iterator::enumerate(values.iter()) {
                    ::planus::WriteAsPrimitive::write(
                        v,
                        ::planus::Cursor::new(unsafe { &mut *bytes.add(i) }),
                        buffer_position - (12 * i) as u32,
                    );
                }
            }
        }
    }
}
