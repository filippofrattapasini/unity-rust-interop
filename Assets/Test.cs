using System;
using RustNative;
using UnityEditor;
using UnityEngine;


public class Test : MonoBehaviour
{
    public static void Execute()
    {
        Counter newCounter = new Counter(new Args());

        newCounter.Increment();
        
        UInt32[] array = { 1, 2, 3, 4, 5, 6, 7, 8 };
        newCounter.IncrementByMany(array);

        var poses = newCounter.GetPositions();

        foreach (var pose in poses) 
            Debug.Log($"{pose.x}, {pose.y}");
        
        Debug.Log(newCounter.Snapshot.val);
    }
}

#if UNITY_EDITOR
[CustomEditor(typeof(Test))]
public class TestNative_Editor : Editor
{
    public override void OnInspectorGUI()
    {
        base.OnInspectorGUI();

        if (GUILayout.Button("Execute")) 
            Test.Execute();
    }
}
#endif