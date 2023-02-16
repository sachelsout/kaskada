package kaskadav1alpha

import (
	"database/sql/driver"
	"fmt"

	"google.golang.org/protobuf/proto"
)

func (x *CompileResponse) Value() (driver.Value, error) {
	return proto.Marshal(x)
}

func (x *CompileResponse) Scan(src interface{}) error {
	if src == nil {
		return nil
	}
	if b, ok := src.([]byte); ok {
		if err := proto.Unmarshal(b, x); err != nil {
			return err
		}
		return nil
	}
	return fmt.Errorf("unexpected type %T", src)
}
