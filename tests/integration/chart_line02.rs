// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2024, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{Chart, ChartType, Workbook, XlsxError};

// Create rust_xlsxwriter file to compare against Excel file.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    // Add some test data for the chart(s).
    worksheet.write_column(0, 0, [1, 2, 3, 4, 5])?;
    worksheet.write_column(0, 1, [6, 8, 6, 4, 2])?;

    let mut chart = Chart::new(ChartType::Line);
    chart.set_axis_ids(63593856, 63612032);
    chart.set_axis2_ids(63615360, 63613568);

    chart.add_series().set_values(("Sheet1", 0, 0, 4, 0));

    chart
        .add_series()
        .set_values(("Sheet1", 0, 1, 4, 1))
        .set_y2_axis(true);

    worksheet.insert_chart(8, 4, &chart)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_chart_line02() {
    let test_runner = common::TestRunner::new()
        .set_name("chart_line02")
        .set_function(create_new_xlsx_file)
        .ignore_elements("xl/workbook.xml", "<fileVersion")
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}