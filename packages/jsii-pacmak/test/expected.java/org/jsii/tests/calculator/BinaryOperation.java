package org.jsii.tests.calculator;
/**
 * Represents an operation with two operands.
 */
@org.jsii.Jsii(module = org.jsii.tests.calculator.$Module.class, fqn = "jsii$jsii_calc$.BinaryOperation")
public abstract class BinaryOperation extends org.jsii.tests.calculator.lib.Operation implements org.jsii.tests.calculator.lib.IFriendly {
    protected BinaryOperation(final org.jsii.JsiiObject.InitializationMode mode) {
        super(mode);
    }
    /**
     * Say hello!
     */
    public java.lang.String hello() {
        return this.jsiiCall("hello", java.lang.String.class);
    }
    /**
     * @param lhs Left-hand side operand
     * Left-hand side operand
     */
    public org.jsii.tests.calculator.lib.Value getLhs() {
        return this.jsiiGet("lhs", org.jsii.tests.calculator.lib.Value.class);
    }
    /**
     * @param rhs Right-hand side operand
     * Right-hand side operand
     */
    public org.jsii.tests.calculator.lib.Value getRhs() {
        return this.jsiiGet("rhs", org.jsii.tests.calculator.lib.Value.class);
    }
}
