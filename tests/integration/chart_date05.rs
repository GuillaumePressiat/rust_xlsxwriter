// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2023, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{Chart, ChartType, ExcelDateTime, Format, Workbook, XlsxError};

// Create rust_xlsxwriter file to compare against Excel file.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();
    worksheet.set_column_width(0, 12)?;

    let date_format = Format::new().set_num_format_index(14);

    // Add some test data for the chart(s).
    let dates = [
        ExcelDateTime::parse_from_str("2013-01-01")?,
        ExcelDateTime::parse_from_str("2013-01-02")?,
        ExcelDateTime::parse_from_str("2013-01-03")?,
        ExcelDateTime::parse_from_str("2013-01-04")?,
        ExcelDateTime::parse_from_str("2013-01-05")?,
        ExcelDateTime::parse_from_str("2013-01-06")?,
        ExcelDateTime::parse_from_str("2013-01-07")?,
        ExcelDateTime::parse_from_str("2013-01-08")?,
        ExcelDateTime::parse_from_str("2013-01-09")?,
        ExcelDateTime::parse_from_str("2013-01-10")?,
    ];
    let numbers = [10, 30, 20, 40, 20, 60, 50, 40, 30, 30];

    worksheet.write_column_with_format(0, 0, dates, &date_format)?;
    worksheet.write_column(0, 1, numbers)?;

    let mut chart = Chart::new(ChartType::Line);
    chart.set_axis_ids(45937408, 45939328);

    chart
        .add_series()
        .set_categories(("Sheet1", 0, 0, 9, 0))
        .set_values(("Sheet1", 0, 1, 9, 1));

    chart.x_axis().set_num_format("dd/mm/yyyy");
    chart.x_axis().set_num_format_linked_to_source(true);
    chart.x_axis().set_text_axis(true);

    worksheet.insert_chart(8, 4, &chart)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_chart_date05() {
    let test_runner = common::TestRunner::new()
        .set_name("chart_date05")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}