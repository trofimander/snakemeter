from unittest import TestCase

import _snakemeter
import sys

class NativeModuleTest(TestCase):
    def test_frames_count(self):
        self.assertEqual(len(sys._current_frames()), _snakemeter.current_frames_count())

    def test_top_frame(self):
        self.assertEqual(1, len(sys._current_frames()))

        frame = sys._current_frames().items()[0][1]

        # frame filename and name
        self.assertEqual((frame.f_code.co_filename, frame.f_code.co_name),
                         _snakemeter.get_top_frame()[:2])

        # line number of current frame
        self.assertEqual(sys._current_frames().items()[0][1].f_lineno, _snakemeter.get_top_frame()[2])
