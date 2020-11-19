package main

import (
	"fmt"
	"reflect"

	"math"
	"os"
	"strings"
	"testing"

	calc "github.com/aws-cdk/jsii/jsii-calc/go/jsiicalc"
	calclib "github.com/aws-cdk/jsii/jsii-calc/go/scopejsiicalclib"

	"github.com/aws-cdk/jsii/jsii-experimental"
)

func TestMain(m *testing.M) {
	status := m.Run()
	jsii.Close()
	os.Exit(status)
}

func initCalculator() calc.CalculatorIface {
	calculatorProps := calc.CalculatorProps{InitialValue: float64(10), MaximumValue: math.MaxFloat64}
	return calc.NewCalculator(&calculatorProps)
}

func TestCalculator(t *testing.T) {
	// Object creation
	t.Run("Object creation", func(t *testing.T) {
		calculator := initCalculator()
		if reflect.ValueOf(calculator).IsZero() {
			t.Errorf("Expected calculator object to be valid")
		}
	})

	t.Run("Property access", func(t *testing.T) {
		calculator := initCalculator()
		expected := float64(10)
		actual := calculator.GetValue()
		if actual != expected {
			t.Errorf("Expected: %f; Actual %f;", expected, actual)
		}
	})

	t.Run("Property mutation", func(t *testing.T) {
		calculator := initCalculator()
		var newVal float64 = 12345
		currentProps := calclib.NewNumber(newVal)
		calculator.SetCurr(currentProps)
		actual := calculator.GetValue()
		if newVal != actual {
			t.Errorf("Expected: %f; Actual %f;", newVal, actual)
		}
	})

	t.Run("Method with side effect", func(t *testing.T) {
		initial, factor := float64(10), float64(3)
		calculator := calc.NewCalculator(&calc.CalculatorProps{InitialValue: initial, MaximumValue: math.MaxFloat64})
		calculator.Mul(factor)
		expectedProduct := initial * factor
		actualProduct := calculator.GetValue()
		if actualProduct != expectedProduct {
			t.Errorf("Expected quotient: %f; Actual %f;", expectedProduct, actualProduct)
		}
	})

	t.Run("Method with interface{} return type", func(t *testing.T) {
		calculator := initCalculator()
		expectedTypeName := "Calculator"
		actualTypeName := calculator.TypeName()
		// JSII tells us this return value is an "any" type. Therefore the
		// value received by go is type `interface{}` and can be further
		// specialized using reflection.
		switch retType := actualTypeName.(type) {
		case string:
			if actualTypeName != expectedTypeName {
				t.Errorf("Expected type name: %s; Actual %s", expectedTypeName, actualTypeName)
			}
		default:
			t.Errorf("Expected type: %s; Actual type: %s", "string", retType)
		}
	})

	t.Run("Method with args and string return type", func(t *testing.T) {
		calculator := initCalculator()
		lhs, rhs := 10, 3
		calculator.SetCurr(calc.NewMultiply(
			calclib.NewNumber(float64(lhs)),
			calclib.NewNumber(float64(rhs)),
		))
		// expectedString := "(10 * 3)"
		expectedString := fmt.Sprintf("(%d * %d)", lhs, rhs)
		actualString := calculator.ToString()
		if actualString != expectedString {
			t.Errorf("Expected string: %s; Actual %s;", expectedString, actualString)
		}
	})
}

func TestUpcasingReflectable(t *testing.T) {
	delegate := make(map[string]interface{})
	key, val := "key1", "value1"
	delegate[key] = val
	upReflectable := calc.NewUpcasingReflectable(delegate)
	entries := upReflectable.GetEntries()

	if len(entries) != 1 {
		t.Errorf("Entries expected to have length of: 1; Actual: %d", len(entries))
	}

	entry := entries[0]
	upperKey := strings.ToUpper(key)
	actualKey, actualVal := entry.GetKey(), entry.GetValue()
	if actualKey != upperKey {
		t.Errorf("Expected Key: %s; Received Key: %s", upperKey, actualKey)
	}

	if actualVal != val {
		t.Errorf("Expected Value: %s; Received Value: %s", val, actualVal)
	}
}

func TestAllTypes(t *testing.T) {
	allTypes := calc.NewAllTypes()

	t.Run("Array property", func(t *testing.T) {
		expected1, expected2 := "val1", "val2"
		allTypes.SetArrayProperty([]string{expected1, expected2})
		actual := allTypes.GetArrayProperty()
		actual1, actual2 := actual[0], actual[1]

		if actual1 != expected1 || actual2 != expected2 {
			t.Errorf("Expected Values: %s, %s; Received: %s, %s", expected1, expected2, actual1, actual2)
		}
	})

	t.Run("Any property", func(t *testing.T) {
		key, val := "key1", "val1"
		expected := make(map[string]string)
		expected[key] = val
		allTypes.SetAnyProperty(expected)

		actual := allTypes.GetAnyProperty()
		actualVal := reflect.ValueOf(actual)
		switch actualVal.Kind() {
		case reflect.Map:
			extractedVal := reflect.ValueOf(actualVal.MapIndex(reflect.ValueOf(key)).Interface()).String()
			if extractedVal != val {
				t.Errorf("Expected map: %s; received: %s", expected, actual)
			}
		default:
			t.Errorf("Expected type: %s; Actual type: %s", "map[string]string", actualVal.Type())
		}
	})
}
