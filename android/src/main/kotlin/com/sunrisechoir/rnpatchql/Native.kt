package com.sunrisechoir.rnpatchql

import com.sunrisechoir.rngraphql.ProcessMutation

class Patchql {
    var patchqlPointer: Long = 0

    init {
        System.loadLibrary("patchql")
        this.initLibrary()
    }

    fun new(offsetLogPath: String, databasePath: String, publicKey: String, privateKey: String) {
        if (patchqlPointer > 0) {
            this.patchqlDestroy(patchqlPointer)
        }
        patchqlPointer = this.patchqlNew(offsetLogPath = offsetLogPath, databasePath = databasePath, publicKey = publicKey, privateKey = privateKey)
    }

    fun query(query: String, callback: (Result<String>) -> Unit) {

        this.patchqlQueryAsync(patchqlPointer, query) { err, result ->
            if (err != null) {
                var throwable = Throwable(err)
                callback(Result.failure(throwable))
            } else {
                callback(Result.success(result.orEmpty()))
            }
        }
    }

    protected fun finalize() {
        if (patchqlPointer > 0) {
            this.patchqlDestroy(patchqlPointer)
        }
    }

    private external fun patchqlNew(offsetLogPath: String, databasePath: String, publicKey: String, privateKey: String): Long
    private external fun patchqlDestroy(contextPointer: Long)
    private external fun patchqlQueryAsync(contextPointer: Long, query: String, callback: (String?, String?) -> Unit)
    private external fun initLibrary()
}