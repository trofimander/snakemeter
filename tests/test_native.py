from unittest import TestCase

import _snakemeter
import sys

class NativeModuleTest(TestCase):
    def test_frames_count(self):
        self.assertEqual(len(sys._current_frames()), _snakemeter.current_frames_count())