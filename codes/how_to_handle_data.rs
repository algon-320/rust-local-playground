//# lazy_static = "1.4.0"
//---- Put dependencies above ----
#![allow(dead_code, unused_variables)]
#![feature(scoped_threads)]
#![feature(negative_impls)]

use std::rc::Rc;
use std::sync::Arc;

struct T;
impl T {
    fn imm_method(&self) {}
    fn mut_method(&mut self) {}
}

fn imm_ref(r: &T) {}
fn mut_ref(r: &mut T) {}
fn imm_chain<'a>(r: &'a T) -> &'a T {
    r
}
fn mut_chain<'a>(r: &'a mut T) -> &'a mut T {
    r
}
fn consume_val(val: T) {}
fn consume_box(ptr: Box<T>) {}
fn consume_rc(ptr: Rc<T>) {}
fn consume_arc(ptr: Arc<T>) {}

fn stack() {
    // シングルスレッド
    {
        let val1: T = T;
        // スタック上に置かれたval1への参照
        imm_ref(&val1);
        val1.imm_method();

        // コンパイルエラー: immutableな値から&mutを作ることはできない
        // mut_ref(&mut val1);
        // val1.mut_method();

        // スタック上に置かれたval1をムーブ
        consume_val(val1);

        // ---------------------------------

        let mut val2: T = T;

        // スタック上に置かれたval2への参照
        imm_ref(&val2);
        val2.imm_method();

        // スタック上に置かれたval2の可変参照
        mut_ref(&mut val2);
        val2.mut_method();

        // スタック上に置かれたval2をムーブ
        consume_val(val2);

        // ---------------------------------

        let val3 = T;

        let r1: &T = imm_chain(&val3); // r1 は val3 への参照

        // immutableな参照は複数作ることができる (r1 と &val3)
        imm_ref(&val3);

        let r2: &T = imm_chain(r1); // r2 も val3 への参照
        imm_ref(r2);

        // ---------------------------------

        let mut val4 = T;

        let r1: &mut T = mut_chain(&mut val4);

        // - コンパイルエラー
        // - mutableな参照は複数作れない
        // mut_ref(&mut val4); // &mut T として r1 が既に存在する

        let r2: &mut T = mut_chain(r1);
        mut_ref(r2);
    }

    // マルチスレッド
    {
        let val1 = T;
        std::thread::spawn(move || {
            // - クロージャへムーブされた値への参照 (新しいスレッドのスタック上)
            // - T: Send が必要
            imm_ref(&val1);
            val1.imm_method();
        });

        // ---------------------------------

        let val2 = T;
        std::thread::scope(|s| {
            // scopeは生成されたスレッドを自動的にjoinするため、
            // 呼び出したスレッドのスタック上のデータを安全に参照できる。
            //
            // std::thread::spawn() はjoinするタイミングが自由な代わりに、
            // 参照をキャプチャするには'staticのライフタイムが必要。
            // (joinする前に関数を抜けるとスタックが破壊されてしまうため、
            // スタック上のデータを参照できない。)

            s.spawn(|| {
                // メインスレッドのスタック上に置かれたval2への参照
                imm_ref(&val2);
            });
        });
        imm_ref(&val2);

        // ---------------------------------

        let mut val3 = T;
        std::thread::scope(|s| {
            s.spawn(|| {
                mut_ref(&mut val3);
            });
        });
        mut_ref(&mut val3);
    }
}

fn heap() {
    // シングルスレッド
    {
        // - ヒープ上に置かれたTの値へのポインタ
        // - Tをスタック上に構築した後にヒープ上にムーブされる
        // - ptr1 はポインタであり、Boxの値自体はスタック上にある
        // - Box::clone()では新たにヒープを確保し、T::clone()で複製する
        let ptr1: Box<T> = Box::new(T);

        // - ヒープ上に置かれたTへの参照
        // - std::ops::Deref<Target = T>のおかげで&Box<T>から&Tを得られる
        imm_ref(&ptr1);
        imm_ref(&*ptr1);
        ptr1.imm_method();

        // ---------------------------------

        let mut ptr2 = Box::new(T);

        // - ヒープ上に置かれたTの借用
        // - std::ops::DerefMut のおかげで&mut Box<T>から&mut Tを得られる
        mut_ref(&mut ptr2);
        mut_ref(&mut *ptr2);
        ptr2.mut_method();

        // ---------------------------------

        let ptr3 = Box::new(T);

        // - ヒープ上に置かれたTをムーブ
        // - *box_ptr は通常のderefとは異なる特別な操作 (deref move)
        // - ヒープメモリは開放され、スタックにTが移動する
        consume_val(*ptr3);

        // ---------------------------------

        let ptr4 = Box::new(T);

        // - Boxポインタ自体をムーブ
        // - スタック上の値をムーブしているだけ
        // - ヒープ上のデータはそのまま
        consume_box(ptr4);

        // ---------------------------------

        // - ヒープ上に置かれたTの値へのポインタ
        // - 参照カウントで管理される
        // - cloneではポインタがコピーされるだけ (参照先は同じヒープ上のデータ)
        let ptr5: Rc<T> = Rc::new(T);
        consume_rc(ptr5.clone()); // TはCloneを実装していないことに注意

        // - ヒープ上に置かれたTへの参照
        // - std::ops::Deref<Target = T>のおかげで&Rc<T>から&Tを得られる
        imm_ref(&ptr5);
        ptr5.imm_method();

        // ---------------------------------

        let mut ptr6 = Rc::new(T);

        // - コンパイルエラー
        // - std::ops::DerefMut は実装されないので、&mut Rc<T>から&mut Tを得ることはできない
        // mut_ref(&mut ptr6);

        // - 参照カウントが1の場合にのみ Some(&mut T) を得ることができる
        // - 参照カウントが2以上ならNone
        let r1: Option<&mut T> = Rc::get_mut(&mut ptr6);
        mut_ref(r1.expect("ref-count is 1"));

        // ---------------------------------

        // - ヒープ上に置かれたTの値へのポインタ
        // - atomicな参照カウントで管理される
        // - cloneではポインタがコピーされるだけ (参照先は同じヒープ上のデータ)
        // - Rcと異なりSendが実装されるため、別のスレッドに渡して共有できる
        let mut ptr7: Arc<T> = Arc::new(T);

        // - シングルスレッド環境ではRcと同じように使うことができる
        // - atomicなカウンタを用いる分だけオーバーヘッドが発生する可能性がある
        consume_arc(ptr7.clone());
        imm_ref(&ptr7);
        ptr7.imm_method();
        let r1: Option<&mut T> = Arc::get_mut(&mut ptr7);
        mut_ref(r1.expect("ref-count is 1"));
    }

    // マルチスレッド
    {
        let ptr1 = Arc::new(T);

        // Arcポインタをclone
        let ptr2 = ptr1.clone();
        std::thread::spawn(move || {
            // - ptr2 はクロージャへムーブされる
            // - ArcはSendを実装している
            // - RcはSendを実装していないため同じことをしようとするとエラー

            // - ヒープ上のTへの参照
            imm_ref(&ptr2);
        });

        // ヒープ上のTへの参照 (別のスレッドと共有している)
        imm_ref(&ptr1);

        // ---------------------------------

        let ptr3 = Box::new(T);
        std::thread::scope(|s| {
            s.spawn(|| {
                // メインスレッドのスタックに置かれたptr3が指すヒープ上のTへの参照
                imm_ref(&ptr3);
            });
        });
        // ヒープ上のTへの参照 (別のスレッドは既にjoinされているため共有されていない)
        imm_ref(&ptr3);
    }
}

fn static_data() {
    // シングルスレッド
    {
        // - 静的領域に置かれたTの値
        // - constな式でのみ初期化できる
        // - T: Sync が必要
        static VAL: T = T;

        // 静的領域に置かれたTへの参照 (&'static T)
        imm_ref(&VAL);
        VAL.imm_method();

        // - コンパイルエラー
        // - staticで定義された値から &mut T を作ることはできない
        // mut_ref(&mut VAL);
        // VAL.mut_method();

        // - コンパイルエラー
        // - staticで定義された値をムーブできない
        // consume_val(VAL);

        // ---------------------------------

        struct U;
        impl !Sync for U {}

        // コンパイルエラー (Sync が必要)
        // static XXXX: X = X;

        // ---------------------------------

        // - 静的領域に置かれたTの値
        // - static mutで定義された値へのアクセスはunsafe
        // - Rustのルールを侵害する可能性があるためunsafe
        //   1. &T は複数存在できるが、&mut T と共存できない
        //   2. &mut T は同時に2つ以上存在してはならない
        static mut MUT_VAL: T = T;

        // 静的領域に置かれたTへの参照 (&'static T)
        unsafe { imm_ref(&MUT_VAL) };
        unsafe { MUT_VAL.imm_method() };

        // 静的領域に置かれたTへの可変参照 (&mut 'static T)
        unsafe { mut_ref(&mut MUT_VAL) };
        unsafe { MUT_VAL.mut_method() };

        // - コンパイルエラー
        // - static mutで定義された値をムーブできない
        // consume_val(MUT_VAL);

        // ---------------------------------

        fn compute() -> T {
            T
        }

        // 外部クレート lazy_static
        // - 動的に初期化できる
        // - 内部的には static mut な値を生成して初回アクセス時に初期化する
        lazy_static::lazy_static! {
            static ref LAZY_VAL: T = {
                compute()
            };
        }

        // 静的領域に置かれたTへの参照 (&'static T)
        imm_ref(&LAZY_VAL);
        LAZY_VAL.imm_method();

        // - コンパイルエラー
        // - lazy_static の static ref で定義された値から &mut T を作ることはできない
        // mut_ref(&mut LAZY_VAL);
        // LAZY_VAL.mut_method();
    }

    // マルチスレッド
    {
        static VAL: T = T;

        std::thread::spawn(move || {
            // 静的領域に確保されたTへの参照 (&'static T)
            imm_ref(&VAL);
            VAL.imm_method();
        });

        // 静的領域に確保されたTへの参照 (&'static T)
        // (別のスレッドと共有している)
        imm_ref(&VAL);
        VAL.imm_method();
    }
}

fn interior_mutability() {
    use std::cell::Cell;

    // - スタック上のCell
    // - Copy が必要
    let val1: Cell<i32> = Cell::new(0);

    // Cellの値を取得する
    let inner: i32 = val1.get();
    assert_eq!(0_i32, inner);

    // Cellの値を変更する
    val1.replace(1_i32);
    assert_eq!(1_i32, val1.get());

    // ---------------------------------

    use std::cell::RefCell;

    // - スタック上のRefCell<T>
    // - 任意の型Tに対して使える
    let val2: RefCell<T> = RefCell::new(T);

    // - RefCellの内部のデータ(スタック上)への参照
    // - RefCell内部への可変参照が既に存在する場合にはパニックする
    {
        let r1: std::cell::Ref<'_, T> = val2.borrow();

        // Refは複数あってもOK
        let r2: std::cell::Ref<'_, T> = val2.borrow();

        imm_ref(&r1);
        r1.imm_method();

        imm_ref(&r2);
        r2.imm_method();
    }

    // - RefCellの内部のデータへの可変参照
    // - RefCell内部への別の可変参照が既に存在する場合にはパニックする
    {
        let mut r3: std::cell::RefMut<'_, T> = val2.borrow_mut();

        // パニック
        // let r4: std::cell::Ref<'_, T> = val2.borrow();
        // let r5: std::cell::RefMut<'_, T> = val2.borrow_mut();

        mut_ref(&mut r3);
        r3.mut_method();
    }

    // ---------------------------------

    use std::sync::atomic::{AtomicU8, Ordering};

    // - スタック上のAtomicU8
    // - atomicに操作できる u8
    // - &AtomicU8 であっても値を変更できる
    let val3: AtomicU8 = AtomicU8::new(0_u8);

    // 値を取得する
    // - Orderingについてはドキュメントを参照すること
    // - Orderingの指定の仕方によって、コンパイラの最適化が抑制される可能性がある
    let v: u8 = val3.load(Ordering::SeqCst);
    assert_eq!(0_u8, v);

    // 値を変更する
    val3.store(1_u8, Ordering::SeqCst);
    assert_eq!(1_u8, val3.load(Ordering::SeqCst));

    // - Atomic系の型はSync を実装しているため、スレッド間で共有できる
    // - std::thread::spawnは&'staticを要求するため、Arcと組み合わせて使うことが多い
    let ptr1 = Arc::new(AtomicU8::new(0_u8));
    let ptr2 = ptr1.clone();
    std::thread::spawn(move || {
        ptr2.store(1_u8, Ordering::SeqCst);
    });
    {
        ptr1.store(2_u8, Ordering::SeqCst);
    }
    let v = val3.load(Ordering::SeqCst);
    assert!(v == 1_u8 || v == 2_u8);

    // ---------------------------------

    use std::sync::Mutex;

    // - スタック上のMutex<T>
    // - 任意の型Tに対して使える
    // - マルチスレッドで排他的にTを操作するために用いる
    // - シングルスレッドで使うメリットはない
    let val4: Mutex<T> = Mutex::new(T);

    {
        // 他のスレッドがロックを持ったままパニックした場合にはErrを返す
        let guard: std::sync::MutexGuard<'_, T> = val4.lock().expect("mutex is poisoned");

        // Mutexの内部への参照 (スタック上)
        imm_ref(&guard);
        guard.imm_method();

        // guard がdropされると自動的にロックが解放される
    }

    {
        let mut guard: std::sync::MutexGuard<'_, T> = val4.lock().expect("mutex is poisoned");

        // Mutexの内部への可変参照
        mut_ref(&mut guard);
        guard.mut_method();
    }

    // - MutexはSyncを実装しているため、スレッド間でデータを共有するために用いることができる
    // - std::thread::spawnは&'staticを要求するため、Arcと組み合わせて使うことが多い
    let ptr1: Arc<Mutex<T>> = Arc::new(Mutex::new(T));

    let ptr2 = ptr1.clone();
    std::thread::spawn(move || {
        // 他のスレッドがロックを保持している場合は、解放するまで待つ
        let mut guard = ptr2.lock().unwrap();

        // - Mutex内部への可変参照
        // - ロックの仕組みによって、排他的に操作することができる
        mut_ref(&mut guard);
    });

    // - static ならArcは不要
    // - Mutex::new がconstではないため、lazy_static などが必要になる
    lazy_static::lazy_static! {
        static ref MUTEX: Mutex<T> = Mutex::new(T);
    }
    std::thread::spawn(|| {
        let mut guard = MUTEX.lock().unwrap();
        mut_ref(&mut guard);
    });
    {
        let mut guard = MUTEX.lock().unwrap();
        mut_ref(&mut guard);
    }
}

fn main() {
    stack();
    heap();
    static_data();
    interior_mutability();
}
