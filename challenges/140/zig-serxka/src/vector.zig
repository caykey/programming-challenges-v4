const std = @import("std");

pub fn Vector(comptime T: type, comptime N: usize) type {
    return struct {
        const Self = @This();

        comp: [N]T,

        pub fn init(values: [N]T) Self {
            var self = Self{
                .comp = undefined,
            };
            std.mem.copy(T, &self.comp, &values);
            return self;
        }

        pub fn zero() Self {
            return std.mem.zeroes(Self);
        }

        pub fn dot(a: Self, b: Self) T {
            comptime var i = 0;
            var sum: T = 0;
            inline while (i < N) : (i += 1)
                sum += a.comp[i] * b.comp[i];
            return sum;
        }

        pub fn cross(a: Self, b: Self) Self {
            if (N != 3)
                @compileError("The cross product can only be computed on Vector with a length of 3");
            return Self.init(.{
                a.comp[1] * b.comp[2] - a.comp[2] * b.comp[1],
                a.comp[2] * b.comp[0] - a.comp[0] * b.comp[2],
                a.comp[0] * b.comp[1] - a.comp[1] * b.comp[0],
            });
        }

        pub fn format(
            self: Self,
            comptime fmt: []const u8,
            options: std.fmt.FormatOptions,
            writer: anytype,
        ) !void {
            _ = fmt;
            _ = options;
            try writer.print("{d}", .{self.comp});
        }
    };
}
