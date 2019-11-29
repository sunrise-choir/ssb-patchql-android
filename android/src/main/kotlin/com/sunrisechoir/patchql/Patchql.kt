package com.sunrisechoir.patchql

class Params(
    val offsetLogPath: String,
    val databasePath: String,
    val publicKey: String,
    val privateKey: String
)

// This is a way of having a Singleton of Patchql. We want this because:
// - memory use
// - we want to enforce a single writer of the sqlite db
class PatchqlManager private constructor(params: Params) {
    private val instance = Patchql()

    init {
        instance.new(params)
    }

    fun query(query: String, callback: (Result<String>) -> Unit) {
        instance.query(query, callback)
    }

    companion object : SingletonHolder<PatchqlManager, Params>(::PatchqlManager)
}


private class Patchql {

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

    fun new(params: Params) {
        new(params.offsetLogPath, params.databasePath, params.publicKey, params.privateKey)
    }

    fun query(query: String, callback: (Result<String>) -> Unit) {

        this.patchqlQueryAsync(patchqlPointer, query) { err, result ->
            if (err != null) {
                val throwable = Throwable(err)
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