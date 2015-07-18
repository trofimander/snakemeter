from unittest import TestCase

import _snakemeter
import sys

class NativeModuleTest(TestCase):
    def test_frames_count(self):
        self.assertEqual(len(sys._current_frames()), _snakemeter.current_frames_count())

    def test_frame_lineno(self):
        self.assertEqual(1, len(sys._current_frames()))

        self.assertEqual(sys._current_frames().items()[0][1].f_lineno, _snakemeter.get_lineno())
