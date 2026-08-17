class HelloWorld {
    private static native String hello(String input);
    private static native byte[] helloByte(byte[] input);
    private static native void factAndCallMeBack(int n, HelloWorld callback);

    private static native long counterNew(HelloWorld callback);
    private static native void counterIncrement(long counter_ptr);
    private static native void counterDestroy(long counter_ptr);

    private static native void asyncComputation(HelloWorld callback);

    static {
        System.loadLibrary("mylib");
    }

    public static void main(String[] args) {

        String output = HelloWorld.hello("josh");
        System.out.println(output);
        byte[] outputByte = HelloWorld.helloByte("byte".getBytes());
        System.out.println(outputByte);


        HelloWorld.factAndCallMeBack(6, new HelloWorld());

        long counter_ptr = counterNew(new HelloWorld());

        for (int i = 0; i < 5; i++) {
          counterIncrement(counter_ptr);
        }

        counterDestroy(counter_ptr);

        System.out.println("Invoking asyncComputation (thread id = " + Thread.currentThread().getId() + ")");
        asyncComputation(new HelloWorld());
    }

    public void factCallback(int res) {
      System.out.println("factCallback: res = " + res);
    }

    public void counterCallback(int count) {
      System.out.println("counterCallback: count = " + count);
    }

    public void asyncCallback(int progress) {
        System.out.println("asyncCallback: thread id = " + Thread.currentThread().getId() + ", progress = " + progress + "%");
    }
}
