;; This file was generated by https://github.com/wasmerio/wasi-tests

(wasi_test "fd_read.wasm"
  (map_dirs ".:test_fs/hamlet")
  (assert_return (i64.const 0))
  (assert_stdout "SCENE IV. The Queen's closet.\n\n    Enter QUEEN GERTRUDE and POLO\n")
)
