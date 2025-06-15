#![no_std]
#![doc = include_str!("../README.md")]

pub use getopts;
use getopts::{HasArg, Occur};

/// [`getopts_options`] `*?` and `+?` support
pub trait GetOptsExt {
    fn optflagreqopt(
        &mut self,
        short_name: &str,
        long_name: &str,
        desc: &str,
        hint: &str,
    ) -> &mut Self;
    fn optflagmultiopt(
        &mut self,
        short_name: &str,
        long_name: &str,
        desc: &str,
        hint: &str,
    ) -> &mut Self;
}
impl GetOptsExt for getopts::Options {
    fn optflagreqopt(
        &mut self,
        short_name: &str,
        long_name: &str,
        desc: &str,
        hint: &str,
    ) -> &mut Self {
        self.opt(short_name, long_name, desc, hint, HasArg::Maybe, Occur::Req)
    }

    fn optflagmultiopt(
        &mut self,
        short_name: &str,
        long_name: &str,
        desc: &str,
        hint: &str,
    ) -> &mut Self {
        self.opt(short_name, long_name, desc, hint, HasArg::Maybe, Occur::Multi)
    }
}

/// Quick create [`getopts::Options`]
///
/// # Syntax
///
/// `[...]` is optional syntax
///
/// *Occur* (default optional):
///
/// - `*` is require multi
/// - `+` is require one
///
/// *HasArg* (default no arg):
///
/// - `=` is has arg
/// - `?=` is has optional arg
///
/// *Option*:
///
/// - `-` *ident*
/// - `--` *ident* \*(`-` *ident*)
/// - `-` *ident* \[`,`] `--` *ident* \*(`-` *ident*)
///
/// *Desc*: *any-token*
///
/// *Syntax*: *Option* \[*Occur*] \[*HasArg* *Desc*] *Desc* `;` \[*Syntax*]
///
/// # Examples
///
/// ```
/// use getopts_macro::{
///     getopts_options,
///     getopts::ParsingStyle,
/// };
///
/// let _options = getopts_options! {
///     -f --file=FILE              "input from file";
///     -p --parse-config*=CONFIG   "parse config";
///     -h --help*                  "help messages";
///        --help-long*             "long help messages";
///     .parsing_style(ParsingStyle::StopAtFirstFree)
/// };
/// ```
#[macro_export]
macro_rules! getopts_options {
    (-$($t:tt)*) => {{
        let mut options = $crate::getopts::Options::new();
        $crate::getopts_options!(@with(options) -$($t)*);
        options
    }};
    (@long $first:ident $($rest:ident)*) => {
        ::core::concat!(
            ::core::stringify!($first),
            $(
                "-",
                ::core::stringify!($rest),
            )*
        )
    };

    (@with($o:ident)) => {};
    (@with($o:ident) . $($t:tt)+) => {
        $o.$($t)+;
    };
    (@with($o:ident) $t1:tt $t2:tt $desc:tt; $($rest:tt)*) => {
        $crate::getopts_options!(@impl($o, $desc) $t1 $t2);
        $crate::getopts_options!(@with($o) $($rest)*);
    };
    (@with($o:ident) $t1:tt $t2:tt $t3:tt $desc:tt; $($rest:tt)*) => {
        $crate::getopts_options!(@impl($o, $desc) $t1 $t2 $t3);
        $crate::getopts_options!(@with($o) $($rest)*);
    };
    (@with($o:ident) $t1:tt $t2:tt $t3:tt $t4:tt $desc:tt; $($rest:tt)*) => {
        $crate::getopts_options!(@impl($o, $desc) $t1 $t2 $t3 $t4);
        $crate::getopts_options!(@with($o) $($rest)*);
    };
    (@with($o:ident) $t1:tt $t2:tt $t3:tt $t4:tt $t5:tt $desc:tt; $($rest:tt)*) => {
        $crate::getopts_options!(@impl($o, $desc) $t1 $t2 $t3 $t4 $t5);
        $crate::getopts_options!(@with($o) $($rest)*);
    };
    (@with($o:ident) $t1:tt $t2:tt $t3:tt $t4:tt $t5:tt $t6:tt $desc:tt; $($rest:tt)*) => {
        $crate::getopts_options!(@impl($o, $desc) $t1 $t2 $t3 $t4 $t5 $t6);
        $crate::getopts_options!(@with($o) $($rest)*);
    };
    (@with($o:ident) $t1:tt $t2:tt $t3:tt $t4:tt $t5:tt $t6:tt $t7:tt $desc:tt; $($rest:tt)*) => {
        $crate::getopts_options!(@impl($o, $desc) $t1 $t2 $t3 $t4 $t5 $t6 $t7);
        $crate::getopts_options!(@with($o) $($rest)*);
    };
    (@with($o:ident) $t1:tt $t2:tt $t3:tt $t4:tt $t5:tt $t6:tt $t7:tt $t8:tt $desc:tt; $($rest:tt)*) => {
        $crate::getopts_options!(@impl($o, $desc) $t1 $t2 $t3 $t4 $t5 $t6 $t7 $t8);
        $crate::getopts_options!(@with($o) $($rest)*);
    };
    (@with($o:ident) $t1:tt $t2:tt $t3:tt $t4:tt $t5:tt $t6:tt $t7:tt $t8:tt $t9:tt $desc:tt; $($rest:tt)*) => {
        $crate::getopts_options!(@impl($o, $desc) $t1 $t2 $t3 $t4 $t5 $t6 $t7 $t8 $t9);
        $crate::getopts_options!(@with($o) $($rest)*);
    };
    (@with($o:ident) $t1:tt $t2:tt $t3:tt $t4:tt $t5:tt $t6:tt $t7:tt $t8:tt $t9:tt $t10:tt $desc:tt; $($rest:tt)*) => {
        $crate::getopts_options!(@impl($o, $desc) $t1 $t2 $t3 $t4 $t5 $t6 $t7 $t8 $t9 $t10);
        $crate::getopts_options!(@with($o) $($rest)*);
    };
    (@with($o:ident) $t1:tt $t2:tt $t3:tt $t4:tt $t5:tt $t6:tt $t7:tt $t8:tt $t9:tt $t10:tt $t11:tt $desc:tt; $($rest:tt)*) => {
        $crate::getopts_options!(@impl($o, $desc) $t1 $t2 $t3 $t4 $t5 $t6 $t7 $t8 $t9 $t10 $t11);
        $crate::getopts_options!(@with($o) $($rest)*);
    };
    (@with($o:ident) $t1:tt $t2:tt $t3:tt $t4:tt $t5:tt $t6:tt $t7:tt $t8:tt $t9:tt $t10:tt $t11:tt $t12:tt $desc:tt; $($rest:tt)*) => {
        $crate::getopts_options!(@impl($o, $desc) $t1 $t2 $t3 $t4 $t5 $t6 $t7 $t8 $t9 $t10 $t11 $t12);
        $crate::getopts_options!(@with($o) $($rest)*);
    };
    (@with($o:ident) $t1:tt $t2:tt $t3:tt $t4:tt $t5:tt $t6:tt $t7:tt $t8:tt $t9:tt $t10:tt $t11:tt $t12:tt $t13:tt $desc:tt; $($rest:tt)*) => {
        $crate::getopts_options!(@impl($o, $desc) $t1 $t2 $t3 $t4 $t5 $t6 $t7 $t8 $t9 $t10 $t11 $t12 $t13);
        $crate::getopts_options!(@with($o) $($rest)*);
    };
    (@with($o:ident) $t1:tt $t2:tt $t3:tt $t4:tt $t5:tt $t6:tt $t7:tt $t8:tt $t9:tt $t10:tt $t11:tt $t12:tt $t13:tt $t14:tt $desc:tt; $($rest:tt)*) => {
        $crate::getopts_options!(@impl($o, $desc) $t1 $t2 $t3 $t4 $t5 $t6 $t7 $t8 $t9 $t10 $t11 $t12 $t13 $t14);
        $crate::getopts_options!(@with($o) $($rest)*);
    };
    (@with($o:ident) $t1:tt $t2:tt $t3:tt $t4:tt $t5:tt $t6:tt $t7:tt $t8:tt $t9:tt $t10:tt $t11:tt $t12:tt $t13:tt $t14:tt $t15:tt $desc:tt; $($rest:tt)*) => {
        $crate::getopts_options!(@impl($o, $desc) $t1 $t2 $t3 $t4 $t5 $t6 $t7 $t8 $t9 $t10 $t11 $t12 $t13 $t14 $t15);
        $crate::getopts_options!(@with($o) $($rest)*);
    };
    (@with($o:ident) $t1:tt $t2:tt $t3:tt $t4:tt $t5:tt $t6:tt $t7:tt $t8:tt $t9:tt $t10:tt $t11:tt $t12:tt $t13:tt $t14:tt $t15:tt $t16:tt $desc:tt; $($rest:tt)*) => {
        $crate::getopts_options!(@impl($o, $desc) $t1 $t2 $t3 $t4 $t5 $t6 $t7 $t8 $t9 $t10 $t11 $t12 $t13 $t14 $t15 $t16);
        $crate::getopts_options!(@with($o) $($rest)*);
    };
    (@with($o:ident) $t1:tt $t2:tt $t3:tt $t4:tt $t5:tt $t6:tt $t7:tt $t8:tt $t9:tt $t10:tt $t11:tt $t12:tt $t13:tt $t14:tt $t15:tt $t16:tt $t17:tt $desc:tt; $($rest:tt)*) => {
        $crate::getopts_options!(@impl($o, $desc) $t1 $t2 $t3 $t4 $t5 $t6 $t7 $t8 $t9 $t10 $t11 $t12 $t13 $t14 $t15 $t16 $t17);
        $crate::getopts_options!(@with($o) $($rest)*);
    };
    (@with($o:ident) $t1:tt $t2:tt $t3:tt $t4:tt $t5:tt $t6:tt $t7:tt $t8:tt $t9:tt $t10:tt $t11:tt $t12:tt $t13:tt $t14:tt $t15:tt $t16:tt $t17:tt $t18:tt $desc:tt; $($rest:tt)*) => {
        $crate::getopts_options!(@impl($o, $desc) $t1 $t2 $t3 $t4 $t5 $t6 $t7 $t8 $t9 $t10 $t11 $t12 $t13 $t14 $t15 $t16 $t17 $t18);
        $crate::getopts_options!(@with($o) $($rest)*);
    };

    (@impl($o:ident, $desc:tt) -$short:ident $(,)? --$($long:ident)-+) => {
        $o.optflag(
            ::core::stringify!($short),
            $crate::getopts_options!(@long $($long)+),
            $desc,
        );
    };
    (@impl($o:ident, $desc:tt) -$short:ident $(,)? --$($long:ident)-+ *) => {
        $o.optflagmulti(
            ::core::stringify!($short),
            $crate::getopts_options!(@long $($long)+),
            $desc,
        );
    };
    (@impl($o:ident, $desc:tt) -$short:ident $(,)? --$($long:ident)-+ = $hint:ident) => {
        $o.optopt(
            ::core::stringify!($short),
            $crate::getopts_options!(@long $($long)+),
            $desc,
            ::core::stringify!($hint),
        );
    };
    (@impl($o:ident, $desc:tt) -$short:ident $(,)? --$($long:ident)-+ * = $hint:ident) => {
        $o.optmulti(
            ::core::stringify!($short),
            $crate::getopts_options!(@long $($long)+),
            $desc,
            ::core::stringify!($hint),
        );
    };
    (@impl($o:ident, $desc:tt) -$short:ident $(,)? --$($long:ident)-+ *= $hint:ident) => {
        $o.optmulti(
            ::core::stringify!($short),
            $crate::getopts_options!(@long $($long)+),
            $desc,
            ::core::stringify!($hint),
        );
    };
    (@impl($o:ident, $desc:tt) -$short:ident $(,)? --$($long:ident)-+ + = $hint:ident) => {
        $o.reqopt(
            ::core::stringify!($short),
            $crate::getopts_options!(@long $($long)+),
            $desc,
            ::core::stringify!($hint),
        );
    };
    (@impl($o:ident, $desc:tt) -$short:ident $(,)? --$($long:ident)-+ += $hint:ident) => {
        $o.reqopt(
            ::core::stringify!($short),
            $crate::getopts_options!(@long $($long)+),
            $desc,
            ::core::stringify!($hint),
        );
    };
    (@impl($o:ident, $desc:tt) -$short:ident $(,)? --$($long:ident)-+ ?= $hint:ident) => {
        $o.optflagopt(
            ::core::stringify!($short),
            $crate::getopts_options!(@long $($long)+),
            $desc,
            ::core::stringify!($hint),
        );
    };
    (@impl($o:ident, $desc:tt) -$short:ident $(,)? --$($long:ident)-+ *?= $hint:ident) => {
        $crate::GetOptsExt::optflagmultiopt(
            &mut $o,
            ::core::stringify!($short),
            $crate::getopts_options!(@long $($long)+),
            $desc,
            ::core::stringify!($hint),
        )
    };
    (@impl($o:ident, $desc:tt) -$short:ident $(,)? --$($long:ident)-+ +?= $hint:ident) => {
        $crate::GetOptsExt::optflagreqopt(
            &mut $o,
            ::core::stringify!($short),
            $crate::getopts_options!(@long $($long)+),
            $desc,
            ::core::stringify!($hint),
        )
    };


    (@impl($o:ident, $desc:tt) -$short:ident) => {
        $o.optflag(
            ::core::stringify!($short),
            "",
            $desc,
        );
    };
    (@impl($o:ident, $desc:tt) -$short:ident *) => {
        $o.optflagmulti(
            ::core::stringify!($short),
            "",
            $desc,
        );
    };
    (@impl($o:ident, $desc:tt) -$short:ident = $hint:ident) => {
        $o.optopt(
            ::core::stringify!($short),
            "",
            $desc,
            ::core::stringify!($hint),
        );
    };
    (@impl($o:ident, $desc:tt) -$short:ident * = $hint:ident) => {
        $o.optmulti(
            ::core::stringify!($short),
            "",
            $desc,
            ::core::stringify!($hint),
        );
    };
    (@impl($o:ident, $desc:tt) -$short:ident *= $hint:ident) => {
        $o.optmulti(
            ::core::stringify!($short),
            "",
            $desc,
            ::core::stringify!($hint),
        );
    };
    (@impl($o:ident, $desc:tt) -$short:ident + = $hint:ident) => {
        $o.reqopt(
            ::core::stringify!($short),
            "",
            $desc,
            ::core::stringify!($hint),
        );
    };
    (@impl($o:ident, $desc:tt) -$short:ident += $hint:ident) => {
        $o.reqopt(
            ::core::stringify!($short),
            "",
            $desc,
            ::core::stringify!($hint),
        );
    };
    (@impl($o:ident, $desc:tt) -$short:ident ?= $hint:ident) => {
        $o.optflagopt(
            ::core::stringify!($short),
            "",
            $desc,
            ::core::stringify!($hint),
        );
    };
    (@impl($o:ident, $desc:tt) -$short:ident *?= $hint:ident) => {
        $crate::GetOptsExt::optflagmultiopt(
            &mut $o,
            ::core::stringify!($short),
            "",
            $desc,
            ::core::stringify!($hint),
        )
    };
    (@impl($o:ident, $desc:tt) -$short:ident +?= $hint:ident) => {
        $crate::GetOptsExt::optflagreqopt(
            &mut $o,
            ::core::stringify!($short),
            "",
            $desc,
            ::core::stringify!($hint),
        )
    };


    (@impl($o:ident, $desc:tt) --$($long:ident)-+) => {
        $o.optflag(
            "",
            $crate::getopts_options!(@long $($long)+),
            $desc,
        );
    };
    (@impl($o:ident, $desc:tt) --$($long:ident)-+ *) => {
        $o.optflagmulti(
            "",
            $crate::getopts_options!(@long $($long)+),
            $desc,
        );
    };
    (@impl($o:ident, $desc:tt) --$($long:ident)-+ = $hint:ident) => {
        $o.optopt(
            "",
            $crate::getopts_options!(@long $($long)+),
            $desc,
            ::core::stringify!($hint),
        );
    };
    (@impl($o:ident, $desc:tt) --$($long:ident)-+ * = $hint:ident) => {
        $o.optmulti(
            "",
            $crate::getopts_options!(@long $($long)+),
            $desc,
            ::core::stringify!($hint),
        );
    };
    (@impl($o:ident, $desc:tt) --$($long:ident)-+ *= $hint:ident) => {
        $o.optmulti(
            "",
            $crate::getopts_options!(@long $($long)+),
            $desc,
            ::core::stringify!($hint),
        );
    };
    (@impl($o:ident, $desc:tt) --$($long:ident)-+ + = $hint:ident) => {
        $o.reqopt(
            "",
            $crate::getopts_options!(@long $($long)+),
            $desc,
            ::core::stringify!($hint),
        );
    };
    (@impl($o:ident, $desc:tt) --$($long:ident)-+ += $hint:ident) => {
        $o.reqopt(
            "",
            $crate::getopts_options!(@long $($long)+),
            $desc,
            ::core::stringify!($hint),
        );
    };
    (@impl($o:ident, $desc:tt) --$($long:ident)-+ ?= $hint:ident) => {
        $o.optflagopt(
            "",
            $crate::getopts_options!(@long $($long)+),
            $desc,
            ::core::stringify!($hint),
        );
    };
    (@impl($o:ident, $desc:tt) --$($long:ident)-+ *?= $hint:ident) => {
        $crate::GetOptsExt::optflagmultiopt(
            &mut $o,
            "",
            $crate::getopts_options!(@long $($long)+),
            $desc,
            ::core::stringify!($hint),
        )
    };
    (@impl($o:ident, $desc:tt) --$($long:ident)-+ +?= $hint:ident) => {
        $crate::GetOptsExt::optflagreqopt(
            &mut $o,
            "",
            $crate::getopts_options!(@long $($long)+),
            $desc,
            ::core::stringify!($hint),
        )
    };
}

#[test]
fn test() {
    let usage = getopts_options! {
        -c --center-rule            "...";
        -i --ignore*=NAME           "...";
        -I --ignore-partial*=NAME   "...";
        -S --fake-source-from=SRC   "...";
        -s --sep?=PATTERN           "...";
        -k --keep*?=PATTERN         "...";
        -h,--help*                  "...";
        -m*                         "...";
           --long                   "...";
           --long-arg=A             "...";
        -j+=J                       "...";
        .parsing_style(getopts::ParsingStyle::StopAtFirstFree)
    }.short_usage("prog");
    assert_eq!(usage, "Usage: prog [-c] [-i NAME].. [-I NAME].. [-S SRC] [-s [PATTERN]] [-k [PATTERN]].. [-h].. [-m].. [--long] [--long-arg A] -j J");
}
