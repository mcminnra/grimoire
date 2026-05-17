const std = @import("std");
const clap = @import("clap");

const check_params = clap.parseParamsComptime(
    \\-h, --help  Display this help and exit.
    \\
);

pub fn checkCmd(io: std.Io, gpa: std.mem.Allocator, iter: *std.process.Args.Iterator) !void {
    // Parse
    var diag = clap.Diagnostic{};
    var res = clap.parseEx(clap.Help, &check_params, clap.parsers.default, iter, .{
        .diagnostic = &diag,
        .allocator = gpa,
    }) catch |err| {
        try diag.reportToFile(io, .stderr(), err);
        return err; // propagate error
    };
    defer res.deinit();

    if (res.args.help != 0)
        std.debug.print("--help\n", .{});

    std.debug.print("Made it", .{});
}
