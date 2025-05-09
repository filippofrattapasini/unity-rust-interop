using System;
using System.Runtime.InteropServices;
using RustNative;

public sealed class CounterHandle : SafeHandle
{
    public CounterHandle() : base(IntPtr.Zero, true) { }

    public CounterHandle(IntPtr handle) : base(IntPtr.Zero, true)
    {
        SetHandle(handle);
    }

    public override bool IsInvalid => handle == IntPtr.Zero;

    protected override bool ReleaseHandle()
    {
        RustNative.Counter.destroyCounter(handle);
        return true;
    }
}

public class Counter : IDisposable
{
    private CounterHandle _handle;

    public Counter(Args args)
    {
        IntPtr ptr = RustNative.Counter.createCounter(args);
        _handle = new CounterHandle(ptr);
    }

    public uint Value => RustNative.Counter.getCounterValue(_handle.DangerousGetHandle());

    public CounterData Snapshot => RustNative.Counter.getCounterData(_handle.DangerousGetHandle());
    
    public uint Increment()
    {
        return RustNative.Counter.incrementCounter(_handle.DangerousGetHandle());
    }

    public uint Decrement()
    {
        return RustNative.Counter.decrementCounter(_handle.DangerousGetHandle());
    }
    
    public uint IncrementBy(uint by)
    {
        return RustNative.Counter.incrementCounterBy(_handle.DangerousGetHandle(), by);
    }

    public uint DecrementBy(uint by)
    {
        return RustNative.Counter.decrementCounterBy(_handle.DangerousGetHandle(), by);
    }

    public uint IncrementByMany(uint[] values)
    {
        return RustNative.Counter.incrementCounterByMany(_handle.DangerousGetHandle(), ref values[0], (uint)values.Length);
    }
    
    public uint DecrementByMany(uint[] values)
    {
        return RustNative.Counter.decrementCounterByMany(_handle.DangerousGetHandle(), ref values[0], (uint)values.Length);
    }
    
    public Vector2[] GetPositions()
    {
        IntPtr ptr = RustNative.Counter.getCounterPositions(_handle.DangerousGetHandle(), out uint len);

        if (ptr == IntPtr.Zero || len == 0)
            return Array.Empty<Vector2>();

        var result = new Vector2[len];
        int size = Marshal.SizeOf<Vector2>();

        for (int i = 0; i < len; i++)
        {
            IntPtr elementPtr = IntPtr.Add(ptr, i * size);
            result[i] = Marshal.PtrToStructure<Vector2>(elementPtr);
        }

        return result;
    }

    public void Dispose()
    {
        _handle?.Dispose();
        _handle = null;
    }
}
