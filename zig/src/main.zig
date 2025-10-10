const std = @import("std");

const aoc = @import("aoc");
const cli = @import("cli");

var config = struct {
    year: u16 = undefined,
    day: ?u8 = null,
}{};

pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    var r = try cli.AppRunner.init(allocator);

    const app = cli.App{
        .command = cli.Command{
            .name = "aoc",
            .target = cli.CommandTarget{
                .subcommands = &.{
                    cli.Command{
                        .name = "download",
                        .options = &.{
                            .{
                                .long_name = "year",
                                .help = "the year to download input for",
                                .value_ref = r.mkRef(&config.year),
                                .required = true,
                            },
                            .{
                                .long_name = "day",
                                .help = "the day to download input for",
                                .value_ref = r.mkRef(&config.day),
                                .required = false,
                            },
                        },
                        .target = cli.CommandTarget{
                            .action = cli.CommandAction{
                                .exec = download_cmd,
                            },
                        },
                    },
                    cli.Command{
                        .name = "run",
                        .options = &.{
                            .{
                                .long_name = "year",
                                .help = "the year to run solutions for",
                                .value_ref = r.mkRef(&config.year),
                                .required = true,
                            },
                            .{
                                .long_name = "day",
                                .help = "the day to run solutions for",
                                .value_ref = r.mkRef(&config.day),
                                .required = false,
                            },
                        },
                        .target = cli.CommandTarget{
                            .action = cli.CommandAction{
                                .exec = run_cmd,
                            },
                        },
                    },
                },
            },
        },
    };

    return r.run(&app);
}

fn download_cmd() !void {
    aoc.download_input(config.year, config.day);
}

fn run_cmd() !void {
    try aoc.run_solutions(config.year, config.day);
}
