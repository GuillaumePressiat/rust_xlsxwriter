// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2024, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{Chart, ChartType, Shape, Workbook, XlsxError};

// Create rust_xlsxwriter file to compare against Excel file.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    // Add some test data for the chart(s).
    worksheet.write_column(0, 0, [1, 2, 3, 4, 5])?;
    worksheet.write_column(0, 1, [2, 4, 6, 8, 10])?;
    worksheet.write_column(0, 2, [3, 6, 9, 12, 15])?;

    let mut chart = Chart::new(ChartType::Column);
    chart.set_axis_ids(61365632, 64275584);
    chart.add_series().set_values("=Sheet1!$A$1:$A$5");
    chart.add_series().set_values("=Sheet1!$B$1:$B$5");
    chart.add_series().set_values("=Sheet1!$C$1:$C$5");

    worksheet.insert_chart(8, 4, &chart)?;

    let textbox = Shape::textbox().set_text("This is some text");

    worksheet.insert_shape(24, 5, &textbox)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_textbox04() {
    let test_runner = common::TestRunner::new()
        .set_name("textbox04")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
