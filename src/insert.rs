use crate::context::Context;

pub fn insert_char(cx: &mut Context, char: char) {
    if let Some(row) = cx.content.get_mut(cx.cursor.row) {
        row.insert(cx.cursor.col, char);
        cx.cursor.col += 1;
    }
}

pub fn pop_char(cx: &mut Context) {
    if let Some(row) = cx.content.get_mut(cx.cursor.row) {
        if cx.cursor.col == 0 {
            return;
        }

        row.remove(cx.cursor.col - 1);
        cx.cursor.col -= 1;
    }
}
