const std = @import("std");

const solutions_2023 = @import("y2023/solutions.zig");
const solutions_2024 = @import("y2024/solutions.zig");

const AocError = error{
    YearNotImplemented,
};

pub fn download_input(year: u16, day: ?u8) void {
    if (day) |d| {
        std.log.info("Downloading input file for {d} day {d}", .{ year, d });
    } else {
        std.log.info("Downloading all input files for {d}", .{year});
    }
}

pub fn run_solutions(year: u16, day: ?u8) !void {
    _ = day;

    switch (year) {
        2023 => try solutions_2023.run_all(),
        2024 => try solutions_2024.run_all(),
        else => return AocError.YearNotImplemented,
    }
}
