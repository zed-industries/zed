/*!
 * \copy
 *     Copyright (c)  2008-2013, Cisco Systems
 *     All rights reserved.
 *
 *     Redistribution and use in source and binary forms, with or without
 *     modification, are permitted provided that the following conditions
 *     are met:
 *
 *        * Redistributions of source code must retain the above copyright
 *          notice, this list of conditions and the following disclaimer.
 *
 *        * Redistributions in binary form must reproduce the above copyright
 *          notice, this list of conditions and the following disclaimer in
 *          the documentation and/or other materials provided with the
 *          distribution.
 *
 *     THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 *     "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 *     LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS
 *     FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE
 *     COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT,
 *     INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING,
 *     BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 *     LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
 *     CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 *     LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN
 *     ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
 *     POSSIBILITY OF SUCH DAMAGE.
 *
 *
 *  ref_list_mgr_svc.h
 *
 *  Abstract
 *      Interface for managing reference picture in svc encoder side
 *
 *  History
 *      09/01/2008 Created
 *      08/07/2009 Ported
 *
 *****************************************************************************/
#if !defined(REFERENCE_PICTURE_LIST_MANAGEMENT_SVC_H__)
#define REFERENCE_PICTURE_LIST_MANAGEMENT_SVC_H__

#include "typedefs.h"
#include "encoder_context.h"
#include "codec_app_def.h"

namespace WelsEnc {

typedef enum {
LTR_DIRECT_MARK = 0,
LTR_DELAY_MARK = 1
} LTR_MARKING_PROCESS_MODE;

typedef enum {
FRAME_NUM_EQUAL    = 0x01,
FRAME_NUM_BIGGER   = 0x02,
FRAME_NUM_SMALLER  = 0x04,
FRAME_NUM_OVER_MAX = 0x08
} COMPARE_FRAME_NUM;

/*
*   reset LTR marking , recovery ,feedback state to default
*/
void ResetLtrState (SLTRState* pLtr);
/*
 *  reset reference picture list
 */
void WelsResetRefList (sWelsEncCtx* pCtx);

/*
 *  update reference picture list
 */
bool WelsUpdateRefList (sWelsEncCtx* pCtx);
/*
 *  build reference picture list
 */
bool WelsBuildRefList (sWelsEncCtx* pCtx, const int32_t kiPOC, int32_t iBestLtrRefIdx);

/*
 *  update syntax for reference base related
 */
void WelsUpdateRefSyntax (sWelsEncCtx* pCtx, const int32_t kiPOC, const int32_t kiFrameType);


/*
* check current mark iFrameNum used in LTR list or not
*/
bool CheckCurMarkFrameNumUsed (sWelsEncCtx* pCtx);
/*
*   decide whether current frame include long term reference mark and update long term reference mark syntax
*/
void WelsMarkPic (sWelsEncCtx* pCtx);

#ifdef LONG_TERM_REF_DUMP
void DumpRef (sWelsEncCtx* ctx);
#endif

class IWelsReferenceStrategy {
 public:
  IWelsReferenceStrategy() {};
  virtual ~IWelsReferenceStrategy() { };

  static IWelsReferenceStrategy* CreateReferenceStrategy (sWelsEncCtx* pCtx, const EUsageType keUsageType,
      const bool kbLtrEnabled);
  virtual bool BuildRefList (const int32_t iPOC, int32_t iBestLtrRefIdx) = 0;
  virtual void MarkPic() = 0;
  virtual bool UpdateRefList() = 0;
  virtual void EndofUpdateRefList() = 0;
  virtual void AfterBuildRefList() = 0;

 protected:
  virtual void Init (sWelsEncCtx* pCtx) = 0;
};

class  CWelsReference_TemporalLayer : public IWelsReferenceStrategy {
 public:
  virtual bool BuildRefList (const int32_t iPOC, int32_t iBestLtrRefIdx);
  virtual void MarkPic();
  virtual bool UpdateRefList();
  virtual void EndofUpdateRefList();
  virtual void AfterBuildRefList();

  void Init (sWelsEncCtx* pCtx);
 protected:
  sWelsEncCtx* m_pEncoderCtx;

};

class  CWelsReference_Screen : public CWelsReference_TemporalLayer {
 public:
  virtual bool BuildRefList (const int32_t iPOC, int32_t iBestLtrRefIdx);
  virtual void MarkPic();
  virtual bool UpdateRefList();
  virtual void EndofUpdateRefList();
  virtual void AfterBuildRefList();
};

class  CWelsReference_LosslessWithLtr : public CWelsReference_Screen {
 public:
  virtual bool BuildRefList (const int32_t iPOC, int32_t iBestLtrRefIdx);
  virtual void MarkPic();
  virtual bool UpdateRefList();
  virtual void EndofUpdateRefList();
};
}
#endif//REFERENCE_PICTURE_LIST_MANAGEMENT_SVC_H__
