// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qtime.h");
        type QTime = crate::QTime;

        include!("cxx-qt-lib/qset.h");
        type QSet_QTime = crate::QSet<QTime>;
    }

    unsafe extern "C++" {
        #[rust_name = "cxx_clear"]
        fn clear(self: &mut QSet_QTime);
        #[rust_name = "cxx_contains"]
        fn contains(self: &QSet_QTime, _: &QTime) -> bool;
        #[rust_name = "cxx_remove"]
        fn remove(self: &mut QSet_QTime, _: &QTime) -> bool;
    }

    #[namespace = "rust::cxxqtlib1::qset_QTime"]
    unsafe extern "C++" {
        #[rust_name = "clone"]
        fn qset_clone_QTime(_: &QSet_QTime) -> QSet_QTime;
        #[rust_name = "default"]
        fn qset_default_QTime() -> QSet_QTime;
        #[rust_name = "drop"]
        fn qset_drop_QTime(_: &mut QSet_QTime);
        #[rust_name = "get_unchecked"]
        unsafe fn qset_get_unchecked_QTime(set: &QSet_QTime, pos: usize) -> &QTime;
        #[rust_name = "insert"]
        fn qset_insert_QTime(_: &mut QSet_QTime, _: &QTime);
        #[rust_name = "len"]
        fn qset_len_QTime(_: &QSet_QTime) -> usize;
    }
}
