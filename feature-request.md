# Design Document: Copy Code Context Command для Zed

## 1. Обзор

**Цель:** Добавить нативную команду в Zed, которая копирует выделенный код вместе с метаданными (путь к файлу, номера строк) в буфер обмена в формате, удобном для вставки в контекст LLM.

**Название команды:** `editor: copy code context` (или `editor: copy with context`)

**Доступ:** Command Palette (Cmd+Shift+P) + возможность привязки hotkey

---

## 2. Формат вывода

### 2.1 Базовый формат

```
file:<relative_path>:[<start_line>-<end_line>]
```<language>
<selected_code>
```
```

### 2.2 Примеры

**Однострочное выделение:**
```
file:src/main.rs:[42]
```rust
fn main() {
```
```

**Многострочное выделение:**
```
file:src/components/Button.tsx:[15-28]
```tsx
export function Button({ onClick, children }: ButtonProps) {
  return (
    <button
      className="btn-primary"
      onClick={onClick}
    >
      {children}
    </button>
  );
}
```
```

**Файл вне workspace (fallback на абсолютный путь):**
```
file:/Users/dev/external/config.json:[1-5]
```json
{
  "name": "example",
  "version": "1.0.0"
}
```
```

---

## 3. Функциональные требования

### 3.1 Обязательные (MVP)

| # | Требование | Описание |
|---|------------|----------|
| 1 | Выделенный текст | Копировать только выделенный текст (не весь файл) |
| 2 | Относительный путь | Использовать путь относительно корня workspace |
| 3 | Номера строк | Включать start-end диапазон строк |
| 4 | Язык | Автоматически определять язык для code fence |
| 5 | Clipboard | Копировать результат в системный буфер обмена |
| 6 | Command Palette | Доступ через Cmd+Shift+P |
| 7 | Контекстное меню | Пункт в правом клике рядом с Copy/Cut/Paste |
| 8 | Fallback на строку | При отсутствии выделения копировать текущую строку |

### 3.2 Желательные (Nice-to-have)

| # | Требование | Описание |
|---|------------|----------|
| 7 | Keybinding | Дефолтный hotkey (например, Cmd+Shift+Alt+C) |
| 8 | Множественные выделения | Поддержка multiple selections |
| 9 | Уведомление | Toast/notification при успешном копировании |

### 3.3 Решено - НЕ делаем

| # | Требование | Причина |
|---|------------|---------|
| ❌ | Настройки формата | Хардкодим формат, упрощает реализацию |

---

## 4. Дизайн команды

### 4.1 Регистрация действия

```rust
// В editor/src/actions.rs или аналогичном файле
actions!(editor, [
    // ... существующие действия
    CopyCodeContext,
]);
```

### 4.2 Псевдокод обработчика

```rust
fn copy_code_context(editor: &mut Editor, cx: &mut Context) {
    // 1. Получить буфер
    let buffer = editor.buffer().read(cx);
    let snapshot = buffer.snapshot(cx);

    // 2. Получить выделения или текущую строку
    let selections = editor.selections.all::<Point>(cx);
    let ranges: Vec<Range<Point>> = if selections.iter().all(|s| s.is_empty()) {
        // Нет выделения - используем текущую строку
        selections.iter().map(|s| {
            let row = s.head().row;
            let start = Point::new(row, 0);
            let end = Point::new(row, snapshot.line_len(row));
            start..end
        }).collect()
    } else {
        // Есть выделение - используем его
        selections.iter().map(|s| s.start..s.end).collect()
    };

    // 3. Получить текст
    let snapshot = buffer.snapshot(cx);

    let mut result = String::new();

    for selection in selections {
        // 3. Получить диапазон строк
        let start_row = selection.start.row + 1; // 1-indexed
        let end_row = selection.end.row + 1;

        // 4. Получить путь к файлу
        let file_path = get_relative_path(buffer, cx);

        // 5. Получить язык
        let language = buffer.language().map(|l| l.name()).unwrap_or("text");

        // 6. Получить текст
        let text = buffer.text_for_range(selection.start..selection.end);

        // 7. Форматировать
        let line_range = if start_row == end_row {
            format!("[{}]", start_row)
        } else {
            format!("[{}-{}]", start_row, end_row)
        };

        result.push_str(&format!(
            "file:{}:{}\n```{}\n{}\n```\n",
            file_path,
            line_range,
            language,
            text
        ));
    }

    // 8. Копировать в clipboard
    cx.write_to_clipboard(ClipboardItem::new(result));

    // 9. Показать уведомление (опционально)
    cx.notify();
}

fn get_relative_path(buffer: &Buffer, cx: &Context) -> String {
    if let Some(file) = buffer.file() {
        if let Some(worktree) = file.worktree() {
            // Относительный путь от корня workspace
            return file.path().to_string_lossy().to_string();
        }
        // Fallback на абсолютный путь
        return file.abs_path().to_string_lossy().to_string();
    }
    "untitled".to_string()
}
```

### 4.3 Регистрация в Command Palette

```rust
// В editor/src/editor.rs или где регистрируются команды
cx.register_action(copy_code_context);
```

### 4.4 Keybinding (опционально)

```json
// В default keymap
{
  "context": "Editor && mode == full",
  "bindings": {
    "cmd-shift-alt-c": "editor::CopyCodeContext"
  }
}
```

---

## 5. Edge Cases

| Сценарий | Поведение |
|----------|-----------|
| Нет выделения | **Копировать текущую строку** (строку под курсором) |
| Пустое выделение | Копировать текущую строку |
| Несохранённый файл | Использовать "untitled" как путь |
| Файл вне workspace | Использовать абсолютный путь |
| Множественные выделения | Конкатенировать все блоки с разделителем `\n\n` |
| Очень длинный код | Копировать как есть (без truncation) |
| Бинарный файл | Не поддерживать (только текстовые) |

---

## 6. UX соображения

### 6.1 Обратная связь пользователю

После успешного копирования показать toast notification:
```
"Copied code context to clipboard (15 lines)"
```

### 6.2 Название команды

Варианты для Command Palette:
- `Editor: Copy Code Context` (рекомендуется)
- `Editor: Copy With Context`
- `Editor: Copy Selection With Path`

### 6.3 Расположение в меню

Добавить в контекстное меню редактора (правый клик):
```
Copy                    Cmd+C
Copy Code Context       Cmd+Shift+Alt+C
Cut                     Cmd+X
```

---

## 7. Тестирование

### 7.1 Unit тесты

- [ ] Однострочное выделение → правильный формат
- [ ] Многострочное выделение → правильный диапазон строк
- [ ] Определение языка по расширению файла
- [ ] Относительный путь для файла в workspace
- [ ] Абсолютный путь для файла вне workspace
- [ ] Множественные выделения → все блоки включены
- [ ] Unsaved buffer → "untitled"
- [ ] Без выделения → копируется текущая строка

### 7.2 Integration тесты

- [ ] Копирование через Command Palette работает
- [ ] Копирование через контекстное меню работает
- [ ] Keybinding работает (если добавлен)
- [ ] Clipboard содержит правильный текст
- [ ] Fallback на текущую строку работает

---

## 8. Файлы для модификации (примерные)

Точные пути зависят от структуры Zed, но примерно:

```
crates/editor/src/actions.rs      # Определение действия CopyCodeContext
crates/editor/src/editor.rs       # Реализация обработчика
crates/editor/src/element.rs      # Контекстное меню
assets/keymaps/default-*.json     # Keybinding (опционально)
```

---

## 9. План реализации

1. **Определить действие** `CopyCodeContext` в actions.rs
2. **Реализовать обработчик** с базовой логикой (выделение + fallback на строку)
3. **Зарегистрировать команду** в Command Palette
4. **Добавить в контекстное меню** редактора
5. **Добавить keybinding** (опционально)
6. **Написать тесты**

---

## 10. Принятые решения

| Вопрос | Решение |
|--------|---------|
| Поведение без выделения | Копировать текущую строку |
| Настройки формата | Хардкод, без кастомизации |
| Контекстное меню | Да, добавить |
| Hotkey | На усмотрение (Cmd+Shift+Alt+C как вариант) |
