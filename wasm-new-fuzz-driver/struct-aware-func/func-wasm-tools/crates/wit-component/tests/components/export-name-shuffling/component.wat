(component
  (core module (;0;)
    (type (;0;) (func))
    (func (;0;) (type 0))
    (export "name#a" (func 0))
  )
  (core instance (;0;) (instantiate 0))
  (component (;0;)
    (type (;0;) (record))
    (export (;1;) "foo" (type 0))
  )
  (instance (;0;) (instantiate 0))
  (export (;1;) "other-name" (instance 0))
  (alias export 1 "foo" (type (;0;)))
  (type (;1;) (func (param "f" 0)))
  (alias core export 0 "name#a" (core func (;0;)))
  (func (;0;) (type 1) (canon lift (core func 0)))
  (alias export 1 "foo" (type (;2;)))
  (component (;1;)
    (alias outer 1 1 (type (;0;)))
    (import "a" (func (;0;) (type 0)))
    (alias outer 1 2 (type (;1;)))
    (export (;2;) "foo" (type 1))
    (type (;3;) (func (param "f" 2)))
    (export (;1;) "a" (func 0) (func (type 3)))
  )
  (instance (;2;) (instantiate 1
      (with "a" (func 0))
    )
  )
  (export (;3;) "name" (instance 2))
)