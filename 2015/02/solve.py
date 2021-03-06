#!/usr/bin/env python

from stdlib import aoc

class Day2(aoc.Day):
    def __init__(self):
        super(Day2, self).__init__(__file__)

    def run(self):
        total_paper = 0
        total_ribbon = 0
        lines = self.read_input()
        for dimensions in lines:
        # for dimensions in ['1x1x10', '2x3x4']:
            length, width, height = [int(x) for x in dimensions.split('x')]
            x = length * width
            y = width * height
            z = height * length

            volume = length * width * height
            sides = [length, width, height]
            sides.pop(sides.index(max(sides)))
            smallest_perimiter = sum([ 2 * x for x in sides ])
            total_ribbon += volume + smallest_perimiter

            area = (2 * x + 2 * y + 2 * z)
            smallest_side = min([x, y, z])
            paper = area + smallest_side
            total_paper += paper

        self.log.info('part 1 : %d' % (total_paper))
        self.log.info('part 2 : %d' % (total_ribbon))



if __name__ == '__main__':
    Day2().run()
