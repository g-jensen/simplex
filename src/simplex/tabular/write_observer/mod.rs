#[cfg(test)]
mod test;

use super::{Problem, ProblemObserver, SimplexRow};
use crate::simplex::core::{Coefficients, Variable};
use std::io::Write;

struct RowStrings {
    bv: String,
    coefficients: Vec<String>,
    rhs: String,
    ratio: String,
}

struct ColumnWidths {
    bv: usize,
    vars: Vec<usize>,
    rhs: usize,
    ratio: usize,
}

fn variable_name(var: Variable) -> String {
    format!("x{}", var)
}

fn max_field_width(
    header: &str,
    rows: &[&RowStrings],
    field: impl Fn(&RowStrings) -> usize,
) -> usize {
    rows.iter().map(|r| field(r)).fold(header.len(), usize::max)
}

fn bv_width(rows: &[&RowStrings]) -> usize {
    max_field_width("BV", rows, |r| r.bv.len())
}

fn var_widths(rows: &[&RowStrings], num_vars: usize) -> Vec<usize> {
    (0..num_vars)
        .map(|i| max_field_width(&variable_name(i), rows, |r| r.coefficients[i].len()))
        .collect()
}

fn rhs_width(rows: &[&RowStrings]) -> usize {
    max_field_width("RHS", rows, |r| r.rhs.len())
}

fn ratio_width(rows: &[&RowStrings]) -> usize {
    max_field_width("Ratio", rows, |r| r.ratio.len())
}

fn column_widths(objective_row: &RowStrings, constraint_rows: &[RowStrings]) -> ColumnWidths {
    let all_rows: Vec<&RowStrings> = std::iter::once(objective_row)
        .chain(constraint_rows.iter())
        .collect();
    let num_vars = objective_row.coefficients.len();

    ColumnWidths {
        bv: bv_width(&all_rows),
        vars: var_widths(&all_rows, num_vars),
        rhs: rhs_width(&all_rows),
        ratio: ratio_width(&all_rows),
    }
}

fn format_header(widths: &ColumnWidths) -> String {
    let header_row = RowStrings {
        bv: "BV".to_string(),
        coefficients: (0..widths.vars.len()).map(variable_name).collect(),
        rhs: "RHS".to_string(),
        ratio: "Ratio".to_string(),
    };
    format_row(&header_row, widths)
}

fn all_widths(widths: &ColumnWidths) -> Vec<usize> {
    let mut all = vec![widths.bv];
    all.extend(&widths.vars);
    all.push(widths.rhs);
    all.push(widths.ratio);
    all
}

fn format_separator(widths: &ColumnWidths) -> String {
    let mut separator: String = all_widths(widths)
        .iter()
        .map(|w| format!("|{}", "-".repeat(w + 2)))
        .collect();
    separator.push_str("|\n");
    separator
}

fn format_left_cell(value: &str, width: usize) -> String {
    format!("| {:<width$} ", value)
}

fn format_right_cell(value: &str, width: usize) -> String {
    format!("| {:>width$} ", value)
}

fn format_row(row_data: &RowStrings, widths: &ColumnWidths) -> String {
    let mut row = format_left_cell(&row_data.bv, widths.bv);
    for (i, coeff) in row_data.coefficients.iter().enumerate() {
        row.push_str(&format_right_cell(coeff, widths.vars[i]));
    }
    row.push_str(&format_right_cell(&row_data.rhs, widths.rhs));
    row.push_str(&format_right_cell(&row_data.ratio, widths.ratio));
    row.push_str("|\n");
    row
}

fn stringify_coefficients(coefficients: &Coefficients) -> Vec<String> {
    coefficients.iter().map(|c| c.to_string()).collect()
}

fn stringify_objective(problem: &Problem) -> RowStrings {
    RowStrings {
        bv: "Z".to_string(),
        coefficients: stringify_coefficients(&problem.objective_equation.coefficients),
        rhs: problem.objective_equation.constraint.to_string(),
        ratio: String::new(),
    }
}

fn stringify_constraint(simplex_row: &SimplexRow) -> RowStrings {
    RowStrings {
        bv: variable_name(simplex_row.basic_variable),
        coefficients: stringify_coefficients(&simplex_row.equation.coefficients),
        rhs: simplex_row.equation.constraint.to_string(),
        ratio: simplex_row.ratio.to_string(),
    }
}

pub struct WriteObserver<'a, W: Write> {
    output: &'a mut W,
}

impl<'a, W: Write> WriteObserver<'a, W> {
    pub fn new(output: &'a mut W) -> Self {
        Self { output }
    }
}

impl<W: Write> ProblemObserver for WriteObserver<'_, W> {
    fn observe(&mut self, problem: Problem) {
        let objective_row = stringify_objective(&problem);
        let constraint_rows: Vec<RowStrings> =
            problem.rows.iter().map(stringify_constraint).collect();

        let widths = column_widths(&objective_row, &constraint_rows);

        write!(self.output, "{}", format_header(&widths)).unwrap();
        write!(self.output, "{}", format_separator(&widths)).unwrap();
        write!(self.output, "{}", format_row(&objective_row, &widths)).unwrap();

        for row_data in &constraint_rows {
            write!(self.output, "{}", format_row(row_data, &widths)).unwrap();
        }
    }
}
