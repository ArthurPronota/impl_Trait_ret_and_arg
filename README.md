# `impl Trait` в позиции возврата и аргумента

## Что такое `impl Trait`

**`impl Trait`** — синтаксический сахар для **анонимного типа**, реализующего указанный trait. Используется в **двух** позициях:

| Позиция | Смысл |
|---|---|
| **Аргумент** | Сахар для **generic** (`T: Trait`) |
| **Возврат** | **Один конкретный**, но **неназванный** тип |

## 1. `impl Trait` в позиции **аргумента**

### Синтаксис

```rust
fn log(v: impl std::fmt::Display) {
    println!("{}", v);
}
```

### Эквивалент generic

```rust
fn log<T: std::fmt::Display>(v: T) {
    println!("{}", v);
}
```

**`impl Trait` в аргументе** = **короткая запись** для **generic-параметра** с ограничением.

### Использование

```rust
log("abc");       // ✅
log(42);          // ✅
log(3.14);        // ✅
log(vec![1, 2]);  // ❌ Vec не реализует Display
```

### Разница с generic

**Синтаксически** — то же. **Семантически** — тоже. Только **короче**:

```rust
// Длинно
fn log<T: Display>(v: T) { ... }

// Коротко
fn log(v: impl Display) { ... }
```

### Ограничение

**Нельзя** указать **явно** тип при вызове:

```rust
log::<String>("abc".to_string());   // ❌ нельзя с impl Trait
log::<String>("abc".to_string());   // ✅ с generic
```

## 2. `impl Trait` в позиции **возврата**

### Синтаксис

```rust
fn iter_evens(v: &[i32]) -> impl Iterator<Item = &i32> {
    v.iter().filter(|x| *x % 2 == 0)
}
```

- **`impl Iterator<Item = &i32>`** — возвращает **один конкретный** тип, реализующий `Iterator`.
- **Компилятор знает** тип, но **вы** — нет.
- **Тип фиксирован** внутри функции — **одинаков** для всех вызовов.

### Что **скрыто** за `impl`

Реальный тип:

```rust
std::iter::Filter<
    std::slice::Iter<'_, i32>,
    [closure@src/main.rs:3:13]
>
```

**Длинно** и **некрасиво**. `impl Iterator` **скрывает** это.

### Использование

```rust
let v = vec![1, 2, 3, 4, 5];

for item in iter_evens(&v) {
    println!("{}", item);   // 2, 4
}
```

## Ключевое отличие

| | Аргумент | Возврат |
|---|---|---|
| **Смысл** | Generic (`T: Trait`) | Один конкретный тип |
| **Можно явно** указать тип | ❌ Нет | — |
| **Разные типы** на вызов | ✅ Да | ❌ Нет (**один** тип) |
| **Длина** | Коротко | Коротко |
| **Когда** | Упростить generic | Скрыть сложный тип |

## Почему `impl Trait` **нельзя** возвращать **разные** типы

```rust
fn get(flag: bool) -> impl Iterator<Item = i32> {
    if flag {
        vec![1, 2, 3].into_iter()           // ← тип A
    } else {
        (1..=3)                              // ← тип B
    }
}
```

**Ошибка:**

```
error[E0308]: `if` and `else` have incompatible types
```

**Почему:** `impl Trait` в возврате — **один** тип. `if`/`else` возвращают **разные** типы → **ошибка**.

**Решение:** `Box<dyn Iterator>`:

```rust
fn get(flag: bool) -> Box<dyn Iterator<Item = i32>> {
    if flag {
        Box::new(vec![1, 2, 3].into_iter())
    } else {
        Box::new(1..=3)
    }
}
```

## Lifetime в `impl Trait`

### Проблема: **неявный** захват

```rust
fn iter_evens(v: &[i32]) -> impl Iterator<Item = &i32> {
    v.iter().filter(|x| *x % 2 == 0)
}
```

**`impl Iterator<Item = &i32>`** — **неявно** захватывает **lifetime** `v`:

```rust
fn iter_evens<'a>(v: &'a [i32]) -> impl Iterator<Item = &'a i32> + 'a {
    v.iter().filter(|x| *x % 2 == 0)
}
```

**`+ 'a`** — **скрытый** lifetime bound.

### `'static` — если нет захвата

```rust
fn make_iter() -> impl Iterator<Item = i32> {
    1..=3   // ← нет захвата → 'static
}
```

### Rust 2024: **явный** захват

В **Rust 2024** правила захвата **стали явными**:

```rust
fn iter_evens(v: &[i32]) -> impl Iterator<Item = &i32> + use<'_> {
    v.iter().filter(|x| *x % 2 == 0)
}
```

**`use<'_>`** — **явно** указывает, что **захватывается** lifetime.

## Полный пример

```rust
use std::fmt::Display;

// 1. impl Trait в аргументе — сахар для generic
fn log(v: impl Display) {
    println!("{}", v);
}

// 2. impl Trait в возврате — один конкретный тип
fn iter_evens(v: &[i32]) -> impl Iterator<Item = &i32> {
    v.iter().filter(|x| *x % 2 == 0)
}

// 3. Эквивалент generic
fn log_generic<T: Display>(v: T) {
    println!("{}", v);
}

fn main() {
    let v = vec![1, 2, 3, 4, 5];

    // Перебор
    for item in iter_evens(&v) {
        println!("item: {}", item);
    }
    // item: 2
    // item: 4

    // log
    log("abc");         // abc
    log(42);            // 42
    log(3.14);          // 3.14

    // log_generic
    log_generic("abc"); // abc
    log_generic(42);    // 42
}
```

## Сводная таблица

| Аспект | Аргумент | Возврат |
|---|---|---|
| **Синтаксис** | `fn f(x: impl Trait)` | `fn f() -> impl Trait` |
| **Эквивалент** | `<T: Trait>(x: T)` | Один конкретный тип |
| **Типов на вызов** | Много | **Один** |
| **Явно указать тип** | ❌ | — |
| **Разные типы** в `if`/`else` | — | ❌ |
| **Lifetime** | — | **Неявный** захват |
| **Rust 2024** | — | `use<'_>` |

## Когда использовать

| Ситуация | Решение |
|---|---|
| Упростить generic | `impl Trait` в аргументе |
| Вернуть **сложный** тип (итератор, closure) | `impl Trait` в возврате |
| Вернуть **разные** типы из `if`/`else` | `Box<dyn Trait>` |
| Нужно **явно** указать тип | Generic `<T>` |
| Публичный API | `impl Trait` (скрывает детали) |

## Сводная таблица

| Аспект | `impl Trait` | Generic `<T>` |
|---|---|---|
| **Аргумент** | ✅ Короче | ✅ Явный |
| **Возврат** | ✅ Один тип | ❌ Нельзя |
| **Явно указать тип** | ❌ | ✅ |
| **Turbo-fish** | ❌ | ✅ |
| **Публичный API** | ✅ Скрывает | ⚠️ Раскрывает |

## Итог

- **`impl Trait` в аргументе** — **сахар** для **generic** (`T: Trait`).
- **`impl Trait` в возврате** — **один** конкретный, **неназванный** тип.
- **Скрывает** длинные типы (`Filter<Iter, Closure>`).
- **Тип фиксирован** внутри функции — **одинаков** для всех вызовов.
- **Нельзя** вернуть **разные** типы из `if`/`else` → **`Box<dyn Trait>`**.
- **Lifetime** захваченных аргументов **попадают** в скрытый тип.
- **Rust 2024** — **явный** захват через **`use<'_>`**.
- **В вашем примере:**
  - `iter_evens` → **скрытый** тип `Filter<...>`;
  - `log` → **сахар** для `log<T: Display>`.
- **Правило:** `impl Trait` в **аргументе** — короче generic; в **возврате** — скрыть сложный тип.
